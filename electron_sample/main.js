const electron = require('electron');
const app = electron.app;
const BrowserWindow = electron.BrowserWindow;
const cfdjs = require('cfd-js');

let mainWindow = null;
app.allowRendererProcessReuse = true

app.on('window-all-closed', function() {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('ready', function() {
  console.log('cfdjs.GetSupportedFunction=', cfdjs.GetSupportedFunction());

  // ブラウザ(Chromium)の起動, 初期画面のロード
  mainWindow = new BrowserWindow({width: 800, height: 600});
  mainWindow.loadURL('file://' + __dirname + '/index.html');

  mainWindow.webContents.openDevTools();

  mainWindow.on('closed', function() {
    mainWindow = null;
  });
});
