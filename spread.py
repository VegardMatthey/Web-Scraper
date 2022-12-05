from __future__ import print_function

import os.path

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

import codecs

emails = 25

scopes = ['https://www.googleapis.com/auth/spreadsheets.readonly','https://www.googleapis.com/auth/drive']

range = 'A2:M'
spreadsheet_id = '1RBC1Kry379x4EbQXQbnGsrunsrC2j5RKX24MBqZtdMk'

def main():
    creds = None
    #action='append'
    major_dim='COLUMNS'
    
    filename = "result0.txt"
    
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
    try:
        service = build('sheets', 'v4', credentials=creds)

        sheet = service.spreadsheets()
        result = sheet.values().get(spreadsheetId=spreadsheet_id, range=range, majorDimension=major_dim).execute()
        values = result.get('values', [])

        cell_values = open("parse/data/" + filename, "r").read().split('\n')

        for columns in values:
            if len(columns) > 0 and columns[len(columns) - 1] == cell_values[5]:
                action='update'
                break
    except HttpError as err:
        print(err)

    y = 0
    while y < emails:
        update_values(spreadsheet_id, range, "USER_ENTERED", creds, "result" + str(y) + ".txt")
        y += 1


def update_values(spreadsheet_id, range_name, value_input_option, creds, filename):
    

    try:

        service = build('sheets', 'v4', credentials=creds)
        cell_values = codecs.open("parse/data/" + filename, "r", encoding='utf-8').read().split('\n')

        values = [cell_values]

        body = {
            'values': values
        }
                
       
        result = service.spreadsheets().values().append(spreadsheetId=spreadsheet_id, range=range_name,valueInputOption=value_input_option, body=body).execute()
        #else :
            #result = service.spreadsheets().values().update(spreadsheetId=spreadsheet_id, range=range_name, valueInputOption=value_input_option, body=body).execute()
        
        return result
    except HttpError as error:
        print(f"An error occurred: {error}")
        return error  


if __name__ == '__main__':
    main()