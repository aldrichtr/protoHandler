---
title: "protoHandle.rs : A tool to run scripts and programs based on URI"
---

***This package is currently in the development stage.  Expect changes***

This project is a custom Protocol Handler that invokes a script with the URI
sent by the browser. This allows for seamless integration and automation of
tasks based on specific URI schemes.

## Features

- **Custom Protocol Handling**: Registers a custom protocol handler
- **Script interpreter Integration**: Calls a specified script with the URI
  as a parameter.
- **Automation**: Enables automation of tasks based on custom URI schemes.

## Usage

1. **Define the URI Scheme**: Use the custom URI scheme (e.g., `myprotocol://`)
   in your web application or any other application that supports URI schemes.
2. **Invoke the Protocol**: When the URI is invoked, Windows will call the
   registered PowerShell script with the URI as a parameter.
3. **Handle the URI in PowerShell**: The PowerShell script will receive the URI
   and can process it as needed.
