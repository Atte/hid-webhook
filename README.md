# hid-webhook

Calls a webhook when a specific HID devices produces a key event.

Prevents the event from being passed to the system.

## Environment variables

- `HID_WEBHOOK_DEVICES` - whitespace separated list of HID device file paths to intercept
- `HID_WEBHOOK_URLS` - whitespace separated list of URLs to POST events to
- `HID_WEBHOOK_NOVERIFY` - set to `true` to disable TLS verification

## POST body

```json
{
    "device_path": "/dev/input/event0",
    "code": 272,
    "down": true
}
```
