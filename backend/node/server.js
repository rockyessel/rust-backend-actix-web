const express = require('express');
const puppeteer = require('puppeteer');
const app = express();

app.get('/get-html', async (req, res) => {
  const url = req.query.url;

  const browser = await puppeteer.launch({
    headless: 'new',
    defaultViewport: null,
    args: ['--enable-features=JavaScript'],
  });
  const page = await browser.newPage();
  await page.goto(url);
  await page.waitForSelector('html', { timeout: 1000 });

  const htmlContent = await page.evaluate(() => {
    return document.documentElement.outerHTML;
  });

  await browser.close();

  res.send(htmlContent);
});

app.listen(3000, () => {
  console.log('Node.js server running on port 3000');
});
