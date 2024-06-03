#include <iostream>
#include <Windows.h>
#include <WinUser.h>
#include "DXGICapture.h"
#include "ConvertScale.h"
#include "H264NVENCEncoder.h"

#include "live_push.h"

static bool do_push;

void start_push(void (*push)(const unsigned char* buffer, int size))
{
    int width = GetSystemMetrics(SM_CXSCREEN);
    int height = GetSystemMetrics(SM_CYSCREEN);
    
    DXGICapture capture;
    if (!capture.initialized)
    {
        return;
    }

    do_push = true;
    
    int rgb32Size = ConvertScale::GetImageSize(width, height, AV_PIX_FMT_BGRA, 16);
    unsigned char* rgbBuffer = new unsigned char[rgb32Size];
    unsigned char* h264Buffer = new unsigned char[rgb32Size];

    H264NVENCEncoder encoder;
    encoder.OpenEncoder(width, height, 60, width * height * 3);
    AVPixelFormat pix = encoder.GetInputPixelFormat();

    ConvertScale scale;
    scale.Init(width, height, AV_PIX_FMT_BGRA, width, height, pix);

    int yuvSize = ConvertScale::GetImageSize(width, height, pix, 16);
    unsigned char* yuvBuffer = new unsigned char[yuvSize];

    while (do_push)
    {
        if (!capture.CaptureRgb32(rgbBuffer, rgb32Size))
        {
            continue;
        }
        if (!scale.Convert(rgbBuffer, yuvBuffer))
        {
            continue;
        }
        int ret = encoder.Encoder(yuvBuffer, h264Buffer, rgb32Size);
        if (ret <= 0)
        {
            continue;
        }
        push(h264Buffer, ret);
    }
    delete[] rgbBuffer;
    rgbBuffer = nullptr;
}

void stop_push()
{
    do_push = false;
}