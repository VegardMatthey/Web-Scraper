from selenium import webdriver
from time import sleep
from selenium.webdriver.common.by import By
from selenium.webdriver import Keys, ActionChains
import pyautogui

emails = 25

with open('login.txt') as f:
    login = list(f)
    
driver = webdriver.Firefox()
driver.maximize_window()

driver.get("https://kajabi.com")
driver.implicitly_wait(10)
sleep(1)
submit_button = driver.find_element(by=By.ID, value="onetrust-accept-btn-handler")
submit_button.click()
driver.implicitly_wait(10)
sleep(1)
submit_button = driver.find_element(by=By.ID, value="login")
submit_button.click()
driver.implicitly_wait(10)

text_box = driver.find_element(by=By.ID, value="email")
text_box.send_keys(login[0])
driver.implicitly_wait(20)

text_box = driver.find_element(by=By.ID, value="password")
text_box.send_keys(login[1])
driver.implicitly_wait(10)

submit_button = driver.find_element(by=By.ID, value="btn-login")
submit_button.click()
driver.implicitly_wait(10)

submit_button = driver.find_element(by=By.ID, value="sidenav-marketing")
submit_button.click()
sleep(1); driver.implicitly_wait(10)

driver.get("https://app.kajabi.com/admin/sites/20457/email_campaigns?by_status=sent&page=10")

sleep(1); driver.implicitly_wait(10)

pyautogui.hotkey('ctrl','s'); sleep(1)
pyautogui.write('overview'); sleep(1)
pyautogui.press('tab'); sleep(1)
pyautogui.press('down'); sleep(1)
pyautogui.press('down'); sleep(1)
pyautogui.press('down'); sleep(1)
pyautogui.press('enter'); sleep(1)
pyautogui.press('enter'); sleep(1)

driver.implicitly_wait(10)
pyautogui.press('esc'); sleep(1)
driver.implicitly_wait(10)

sleep(1); driver.implicitly_wait(10)

for i in range(emails):
    for x in range (31 + 3*(emails - 1 - i)):
        ActionChains(driver)\
            .key_down(Keys.TAB)\
            .perform()
    ActionChains(driver)\
        .key_down(Keys.ENTER)\
        .perform()
    
    sleep(4); driver.implicitly_wait(10)

    pyautogui.hotkey('ctrl','s'); sleep(1)
   
    pyautogui.write('html' + str(i)); sleep(1)
    pyautogui.press('tab'); sleep(1)
    pyautogui.press('enter'); sleep(1)
    pyautogui.press('enter'); sleep(1)
    
    driver.back(); driver.implicitly_wait(10)

driver.implicitly_wait(10);
driver.quit()