# notion-calendar-fermyon

A deployment of [Notion-Calendar](https://github.com/trobanga/notion-calendar) with [Spin](https://developer.fermyon.com/spin/v2/index).

## Getting started
To run locally 
``` sh
$ cp env-example .env
```
and adjust the fields.

[Install Spin](https://developer.fermyon.com/spin/v2/install) and run 
``` sh
spin up 
```
Then go to http://127.0.0.1:3000/ical/$USER_ID (or http://127.0.0.1:3000/$USER_ID) to get the iCalendar or to http://127.0.0.1:3000/org/$USER_ID to get the calendar in org-mode format.
