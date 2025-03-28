import { test, expect } from '@playwright/test'

test('click login', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('user').waitFor()
});


test('click register', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('register').click()
  await page.getByTestId('user').waitFor()
});


test('check login success', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('register').click()
  await page.getByTestId('user').fill('checklogin001')
  await page.getByTestId('password').fill('checklogin001')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).waitFor()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).click()
  await page.goto('/authen')
  await page.getByTestId('user').fill('checklogin001')
  await page.getByTestId('password').fill('checklogin001')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).waitFor()

});


test('check register success', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('register').click()
  await page.getByTestId('user').fill('newregistor123')
  await page.getByTestId('password').fill('newregistor123')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).waitFor()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).click()

});


test('check login failure', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('user').fill('asdsdagasdf')
  await page.getByTestId('password').fill('asdsdagasdf')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--error")]').nth(0).waitFor()
  await page.locator('//*[contains(@class,"Toastify__toast--error")]').nth(0).click()

});


test('check register failure', async ({ page }) => {
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('register').click()
  await page.getByTestId('user').fill('checklogin0012')
  await page.getByTestId('password').fill('checklogin0012')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).waitFor()
  await page.locator('//*[contains(@class,"Toastify__toast--success")]').nth(0).click()
  await page.goto('/')
  await page.getByTestId('login').click()
  await page.getByTestId('register').click()
  await page.getByTestId('user').fill('checklogin0012')
  await page.getByTestId('password').fill('checklogin0012')
  await page.getByTestId('submit').click()
  await page.locator('//*[contains(@class,"Toastify__toast--error")]').nth(0).waitFor()
  await page.locator('//*[contains(@class,"Toastify__toast--error")]').nth(0).click()

});



test('check send', async ({ page }) => {
    const lin = "123asd123dsf1"
    await page.goto('/')
    await page.getByTestId('login').click()
    await page.getByTestId('register').click()
    await page.locator('//button[text() = "Sign Up"]').waitFor();
    await page.getByTestId('user').fill(lin)
    await page.getByTestId('password').fill(lin)
    await page.getByTestId('submit').click()
    await page.goto('/authen')
    await page.getByTestId('user').fill(lin)
    await page.getByTestId('password').fill(lin)
    await page.getByTestId('submit').click()
    await page.locator('//button[text() ="Send Money"]').nth(0).waitFor()
    await page.locator(`//*[@data-testid = "table-cell" and text()="${lin}"]/following-sibling::td/button`).click()
    await page.locator('//*[@placeholder = "Enter amount"]').fill("1")
    await page.locator('//button[text() ="Send"]').waitFor()
    await page.locator('//button[text() ="Send"]').click()

});
