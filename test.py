from __future__ import print_function

import google.auth

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

import os.path
import codecs

def update_values(spreadsheet_id, range_name, value_input_option, file_name, _values):

    creds = token()

    try:
        service = build('sheets', 'v4', credentials=creds)

        majorDimension = 'ROWS'
        values = [codecs.open("data/" + file_name, "r", encoding='utf-8').read().split('\n')]

        body = {
            'values': values,
            'majorDimension': majorDimension
        }
        result = service.spreadsheets().values().append(
            spreadsheetId=spreadsheet_id, range=range_name, 
            valueInputOption=value_input_option, body=body).execute()
        print(f"{result.get('updatedCells')} cells updated.")
        return result
    except HttpError as error:
        print(f"An error occurred: {error}")
        return error

def token():
    scopes = ['https://www.googleapis.com/auth/spreadsheets.readonly','https://www.googleapis.com/auth/drive']
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', scopes)
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                'creds.json', scopes)
            creds = flow.run_local_server(port=0)
        with open('token.json', 'w') as token:
            token.write(creds.to_json())
    return creds

if __name__ == '__main__':
    # Pass: spreadsheet_id,  range_name, value_input_option and  _values
    with open('login.txt') as f:
        login = list(f)
    emails = int(login[3])
    for x in range(emails):
        update_values("1RBC1Kry379x4EbQXQbnGsrunsrC2j5RKX24MBqZtdMk",
                      "A1:C2", "USER_ENTERED", "result" + str(x) + ".txt",
                      [
                          ['A', 'B'],
                          ['C', 'D']
                      ])