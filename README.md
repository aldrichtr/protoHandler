---
title: Protocol Handler for Windows
---

This project is a custom Protocol Handler for Windows that invokes a PowerShell
script with the URI sent by the browser. This allows for seamless integration
and automation of tasks based on specific URI schemes.

## Features

- **Custom Protocol Handling**: Registers a custom protocol handler on Windows.
- **PowerShell Integration**: Calls a specified PowerShell script with the URI
  as a parameter.
- **Automation**: Enables automation of tasks based on custom URI schemes.

## Installation

1. **Download the Installer**: Download the installer from the releases page.
2. **Run the Installer**: Execute the installer and follow the on-screen
   instructions.
3. **Register the Protocol Handler**: The installer will automatically register
   the custom protocol handler in the Windows registry.

## Usage

1. **Define the URI Scheme**: Use the custom URI scheme (e.g., `myprotocol://`)
   in your web application or any other application that supports URI schemes.
2. **Invoke the Protocol**: When the URI is invoked, Windows will call the
   registered PowerShell script with the URI as a parameter.
3. **Handle the URI in PowerShell**: The PowerShell script will receive the URI
   and can process it as needed.
