# Auto-Schedule

This project is a webserver built in Rust using the Actix-web framework. The webserver fetches an iCalendar (.ics) file from a given URL, processes the events, and serves the file to be used with Google Calendar. The server auto-updates the .ics file every hour.

## Features

- Fetches an iCalendar file from a specified URL.
- Filters and modifies the events in the .ics file.
- Compares the new .ics file with the previous one to detect changes.
- Serves the .ics file via HTTP.
- Automatically updates the .ics file every hour.

## Requirements

- Rust and Cargo installed on your machine.
- Actix-web and Tokio crates for asynchronous web server and timing.
- Curl command-line tool for fetching the .ics file.

## Installation

1. Clone the repository:
  ```sh
  git clone https://github.com/archways404/auto-schedule.git
  cd auto-schedule
  ```

2. Install the required dependencies:
  ```sh
  cargo build
  ```

3. Make sure the ```fetch_ics.sh``` script is executable:
  ```sh
  chmod +x fetch_ics.sh
  ```

## Usage

1. Run the webserver:
  ```sh
  cargo run
  ```

2. Access the .ics file:
- The .ics file will be served at ```http://localhost:8000/calendar.ics```
- To access and use the file, you are going to need to port-forward.
- After you have port-forwarded, the file will be served at ```http://[YOUR-IP]:[YOUR-PORT]/calendar.ics```.

4. Add the .ics file to Google Calendar:

- Open Google Calendar.

- Click on the "+" button next to "Other calendars" on the left sidebar and select "From URL".

- Enter the URL ```http://[YOUR-IP]:[YOUR-PORT]/calendar.ics``` and click "Add Calendar".

