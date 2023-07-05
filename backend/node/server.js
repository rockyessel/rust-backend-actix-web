import express from 'express';
import { chromium } from 'playwright';

const app = express();

const browser = await chromium.launch({
  headless: false,
});

const page = await browser.newPage({
  // We have to add this flag to enable JavaScript execution
  // on GitHub. waitForFunction() would not work otherwise.
  bypassCSP: true,
});

async function scrapeCrateData(package_name) {
  await page.goto(`https://crates.io/crates/${package_name}`);

  await page.waitForFunction(() => {
    const repoCards = document.querySelector('main');
    return repoCards;
  });

  const crate = await page.$$eval('main', (pack) => {
    return pack?.map((el) => {
      const package_name = el.querySelector(
        'h1._heading_8qtlic > span'
      ).innerHTML;
      const keywords = el.querySelectorAll('ul._keywords_8qtlic > li');
      const toText = (element) => element && element.innerText.trim();
      const time_uploaded = el
        .querySelector('time._date_rj9k29')
        .getAttribute('datetime');
      const license = el.querySelector(
        'div._license_rj9k29 > span > a'
      ).innerText;
      const license_url = el
        .querySelector('div._license_rj9k29 > span > a')
        .getAttribute('href');
      const version = el.querySelector('small').innerText;
      const description = el.querySelector('div._description_8qtlic').innerText;
      const links = el.querySelectorAll('div._content_t2rnmm > a._link_t2rnmm');
      const owners = Array.from(el.querySelectorAll('ul._list_181lzn li')).map(
        (owner) => {
          const image = owner.querySelector('img').getAttribute('src');
          const name = owner.querySelector('span._name_181lzn').innerText;
          const name_url = owner
            .querySelector('a.ember-view')
            .getAttribute('href');
          return { image, name, name_url };
        }
      );

      const getLink = (element) => element && element.getAttribute('href');

      return {
        keywords: Array.from(keywords).map((keyList) => toText(keyList)),
        package_name,
        time_uploaded,
        license: { type: license, url: license_url },
        version,
        description,
        links: Array.from(links).map((link) => getLink(link)),
        owners,
      };
    });
  });

  return crate[0];
}

app.get('/package', async (req, res) => {
  try {
    const package_name = req.query.name;
    if (package_name) {
      const package_ = await scrapeCrateData(package_name);
      await page.waitForTimeout(10000);
      await browser.close();
      res.json(package_);
    } else {
      res.status(404).send('Something went wrong');
    }
  } catch (error) {
    console.log(error);
  }
});

app.listen(3000, () => {
  console.log('Node.js server running on port 3000');
});

//version
//descriptions
//code
//metadata: license, size, homepage, owner, language, github star, total downloads
