from selenium import webdriver
from time import sleep
from selenium.webdriver.common.by import By
from selenium.webdriver import Keys, ActionChains
import pyautogui

pyautogui.PAUSE = 0.5

with open('login.txt') as f:
    login = list(f)
    
emails = int(login[3])
    
driver = webdriver.Firefox()
driver.maximize_window()

driver.get("https://kajabi.com")
driver.implicitly_wait(10); sleep(1)
submit_button = driver.find_element(by=By.ID, value="onetrust-accept-btn-handler"); submit_button.click()
driver.implicitly_wait(10); sleep(1)
submit_button = driver.find_element(by=By.ID, value="login"); submit_button.click()
driver.implicitly_wait(10)

text_box = driver.find_element(by=By.ID, value="email"); text_box.send_keys(login[0])
driver.implicitly_wait(10)

text_box = driver.find_element(by=By.ID, value="password"); text_box.send_keys(login[1])
driver.implicitly_wait(10)

submit_button = driver.find_element(by=By.ID, value="btn-login"); submit_button.click()
driver.implicitly_wait(10)

submit_button = driver.find_element(by=By.ID, value="sidenav-marketing"); submit_button.click()
driver.implicitly_wait(10)

driver.get("https://app.kajabi.com/admin/sites/20457/email_campaigns?by_status=sent&page=1")
driver.implicitly_wait(10)

with open('data/links.txt') as f:
    ids = list(f)
    
driver.get("https://app.kajabi.com/admin/email_broadcasts/"+ ids[0] +"/report"); driver.implicitly_wait(10)
pyautogui.hotkey('command','s')
pyautogui.click(700, 230)
pyautogui.hotkey('command','a')
pyautogui.hotkey('backspace')
pyautogui.write('html' + str(0))
pyautogui.click(700, 350)
pyautogui.click(700, 375)
pyautogui.click(800, 400)

driver.back(); driver.implicitly_wait(10)
for i in range(emails - 1):
    driver.get("https://app.kajabi.com/admin/email_broadcasts/"+ ids[i + 1] +"/report"); driver.implicitly_wait(10)
    pyautogui.hotkey('command','s')
    pyautogui.click(700, 230)
    pyautogui.hotkey('command','a')
    pyautogui.hotkey('backspace')
    pyautogui.write('html' + str(i + 1))
    pyautogui.click(800, 400)
    driver.back(); driver.implicitly_wait(10)

driver.implicitly_wait(10);
driver.quit()