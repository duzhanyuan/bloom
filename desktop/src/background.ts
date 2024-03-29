import {
  app,
  protocol,
  BrowserWindow,
  Tray,
  Menu,
} from 'electron';
import { createProtocol } from './create_protocol';

const isDevelopment = process.env.NODE_ENV !== 'production';

const config = require('./config');

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow: BrowserWindow | null = null;
let tray: Tray | null = null;

// Scheme must be registered before the app is ready
protocol.registerSchemesAsPrivileged([
  { scheme: 'app', privileges: { secure: true, standard: true } },
]);

// show window
function showWindow() {
  // const position = getWindowPosition();
  // mainWindow.setPosition(position.x, position.y, false);
  mainWindow!.show();
  mainWindow!.focus();
}

// toggle window
function toggleWindow() {
  // if (!mainWindow.isVisible()) {
  showWindow();
  // }
}

function createWindow() {
  // create tray icon
  tray = new Tray(config.TRAY_ICON);
  // tray.on('right-click', toggleWindow);
  tray.on('right-click', (event, bounds) => {
    const contextMenu = Menu.buildFromTemplate([
      { label: 'Quit', click: () => { app.quit(); } },
    ]);
    tray!.popUpContextMenu(contextMenu);
    // lastTrayIconBounds = bounds;
  });
  tray.setIgnoreDoubleClickEvents(true);
  tray.on('click', (event) => {
    toggleWindow();
  });
  tray.setToolTip('Bloom');

  // Create the browser window.
  mainWindow = new BrowserWindow({
    title: config.WINDOW_TITLE,
    width: config.WINDOW_DEFAULT_WIDTH,
    height: config.WINDOW_DEFAULT_HEIGHT,
    minWidth: config.WINDOW_MIN_WIDTH,
    minHeight: config.WINDOW_MIN_HEIGHT,
    webPreferences: {
      nodeIntegration: true,
    },
    icon: config.WINDOW_ICON,
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    mainWindow.loadURL(process.env.WEBPACK_DEV_SERVER_URL as string);
    if (!process.env.IS_TEST) {
      mainWindow.webContents.openDevTools();
    }
  } else {
    createProtocol('app');
    // Load the index.html when not in development
    mainWindow.loadURL('app://./index.html');
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

// Quit when all windows are closed.
app.on('window-all-closed', () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  // if (process.platform !== 'darwin') {
  //   app.quit();
  // }
  app.quit();
});

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) {
    createWindow();
  }
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', createWindow);

// Exit cleanly on request from parent process in development mode.
if (isDevelopment) {
  if (process.platform === 'win32') {
    process.on('message', (data) => {
      if (data === 'graceful-exit') {
        app.quit();
      }
    });
  } else {
    process.on('SIGTERM', () => {
      app.quit();
    });
  }
}
