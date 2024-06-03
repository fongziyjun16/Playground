#pragma once

extern "C" {
// #include "ffmpeg/include/libavformat/avformat.h"
#include <libavformat/avformat.h>
// #include "ffmpeg/include/libavcodec/avcodec.h"
#include <libavcodec/avcodec.h>
// #include "ffmpeg/include/libavcodec/qsv.h"
// #include "ffmpeg/include/libavutil/imgutils.h"
#include <libavutil/imgutils.h>
// #include "ffmpeg/include/libavutil/opt.h"
#include <libavutil/opt.h>
// #include "ffmpeg/include/libavutil/hwcontext.h"
#include <libavutil/hwcontext.h>
// #include "ffmpeg/include/libswscale/swscale.h"
#include <libswscale/swscale.h>
}

class H264NVENCEncoder
{
public:
    H264NVENCEncoder();
    ~H264NVENCEncoder();

    bool IsSupported(int width, int height);

    /// <summary>
    /// 初始化编码器
    /// </summary>
    /// <param name="width">h264的宽</param>
    /// <param name="height">h264的高</param>
    /// <param name="frameRate">帧率</param>
    /// <param name="bitrate">码率</param>
    /// <returns></returns>
    bool OpenEncoder(int width, int height, int frameRate, int bitrate);
    void CloseEncoder();

    /// <summary>
    /// 编码一帧
    /// </summary>
    /// <param name="inImageData">编码一帧的数据</param>
    /// <param name="h264Buffer">存放H264数据的Buffer</param>
    /// <param name="h264BufferSize">存放H264数据的Buffer的大小</param>
    /// <returns>编码后的数据大小</returns>
    int Encoder(const unsigned char* inImageData, unsigned char* h264Buffer, int h264BufferSize);

    /// <summary>
    /// 获取输入的像素格式，使用前需要先调用 OpenEncoder。默认为 NV12
    /// </summary>
    /// <returns></returns>
    AVPixelFormat GetInputPixelFormat() { return m_InputPixelFormat; }

private:
    const char* encoder_name = "h264_nvenc";
    
    AVCodecContext* m_pEncoderContext = nullptr;

    AVFrame* m_pAVFrame = nullptr;

    AVPixelFormat m_InputPixelFormat = AV_PIX_FMT_NV12;
};
