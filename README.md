# 視覺化小說遊戲引擎

## 專案簡介

本專案是一套以 .NET 7.0 與 Visual Studio 2022 開發的 Windows 視覺小說遊戲引擎。支援自訂故事腳本、圖片、音樂、音效等多媒體內容，並提供簡單直覺的操作介面。玩家可直接拖曳壓縮檔載入小說劇本，體驗互動式故事。

## 主要功能

- 支援多種圖片格式（bmp, gif, jpg, jpeg, png, tiff）
- 支援背景音樂、音效、影片播放
- 內建多種腳本指令（如：background, music, txt, sound, character, fade, video, exit 等）
- 直覺化 UI 操作，支援全螢幕、視窗切換
- 錯誤處理完善，檔案格式錯誤不會導致程式崩潰
- 可自訂故事腳本，輕鬆擴充內容

## 安裝與執行

### 1. 安裝最新版 .NET Desktop Runtime 7.0

https://dotnet.microsoft.com/en-us/download/dotnet/7.0

### 2. 下載最新的程式壓縮檔，再解壓縮後執行 `WinFormsApp1.exe`

https://drive.google.com/drive/u/0/folders/1TorQdqLAHGfag942-va0NX78k4ut9pHt?usp=share_link

### 3. 下載視覺小說壓縮檔，拖曳到遊戲內(不用解壓)

https://drive.google.com/drive/folders/17RbQoWEW4N9vYSJp0UlB9mfCZtHVbqxY?usp=share_link

可以亂丟檔案，程式會顯示錯誤訊息，不會掛掉.

## 操作說明

- 執行程式後，點擊「開始遊戲」進入故事選單
- 以滑鼠拖曳小說壓縮檔至視窗即可載入
- 遊戲中可使用 F11 切換全螢幕，ESC 或 F11 返回視窗模式
- 遇到不支援的檔案或格式，會顯示錯誤訊息

## 未來可能會填的坑

1. 設定音量.語言

2. 顯示目前時間

3. 儲存遊玩進度

4. 完成 `video` 指令實作
