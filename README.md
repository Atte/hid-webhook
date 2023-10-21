# hid-webhook

Calls a webhook when a specific HID devices produces a key event.

Prevents the event from being passed to the system.

## Environment variables

- `HID_WEBHOOK_DEVICES` - comma separated list of HID device file paths to intercept
- `HID_WEBHOOK_URLS` - comma separated list of URLs to POST events to
- `HID_WEBHOOK_NO_VERIFY` - set to `true` to disable TLS verification (default `false`)
- `HID_WEBHOOK_DOWN` - set to `true` to send `down` events (default `true`)
- `HID_WEBHOOK_UP` - set to `true` to send `up` events (default `false`)
- `HID_WEBHOOK_IGNORE_KEYS` -  comma separated list of key codes to ignore
- `HID_WEBHOOK_TIMEOUT` - timeout for HTTP requests (default `3s`)

## POST body

```json
{
    "device_path": "/dev/input/event0",
    "key": "KEY_A",
    "code": 30,
    "down": true
}
```
