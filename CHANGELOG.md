## 2.0.7

- Merge change from https://github.com/szabodanika/microbin/commits/master/
    - Fix Json db losing already saved pasta's on crash/power failure from @dvdsk
    - Adds a feature no-c-deps which makes microbin easy to crosscompile from @dvdsk
    - SHAREBIN_UPLOADER_PASSWORD was missing from compose.yaml from @secondubly
    - Fix never expire condition from @runofthemillgeek
    - Fix privacyDropdown is null issue from @luochen1990
    - Awnser Range requests and stream files downloads from @dvdsk
    - Set charset=utf-8 for /raw/{id} response from @luk1337
    - fix: division by zero on 32-bit platform from @jixunmoe
    - Fixups from @luk1337

## 2.0.6

- Use the editable Args when creating Pasta

## 2.0.5

- Update dependencies
- Fix SHAREBIN_DATA_DIR attachments to work with absolute path
- Use dotenvy to load .env file
- Remove telemetry
