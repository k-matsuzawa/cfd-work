const builder = require('electron-builder');

builder.build({
    config: {
        'appId': 'local.test.app2',
        'linux':{
             'target': 'zip'
         }
    }
});
