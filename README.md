# Install from local folder

```powershell
py -m pip install --force-reinstall `
   --find-links ..\envencrypt-core\dist `
   "..\envencrypt-core[dpapi]"
```