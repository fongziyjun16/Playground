# Live Recoding

## About

- Capture Screen (DXGI)
- h264 Encode
- RTMP Push

## Structure

### live_client

Recording screen and use `ffmpeg` RTMP push

### live_push

(Release in DLL)

Capture & Encode

Reference: https://github.com/LampsAsarum/CaptureToH264

## Server

- Nginx: https://nginx.org/download/nginx-1.26.1.tar.gz
  - Module: https://github.com/winshining/nginx-http-flv-module

Compile in Ubuntu

## Test

- There are about 3-5s delay.