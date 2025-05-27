[Setup]
AppName=RDSLink
AppVersion={#AppVersion}
DefaultDirName={localappdata}\Programs\RDSLink
DefaultGroupName=RDSLink
DisableProgramGroupPage=yes
OutputDir=..\..
OutputBaseFilename=RDSLinkSetup
Compression=lzma
SolidCompression=yes
PrivilegesRequired=lowest

[Files]
Source: "..\..\target\release\rdslink.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\RDSLink"; Filename: "{app}\rdslink.exe"

[Registry]
Root: HKCU; Subkey: "Environment"; \
    ValueType: expandsz; ValueName: "Path"; \
    AfterInstall: AddToUserPath; Flags: preservestringtype

[Code]
procedure AddToUserPath;
var
  Path: string;
  InstallDir: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    Path := '';

  InstallDir := ExpandConstant('{app}');

  if Pos(';' + InstallDir + ';', ';' + Path + ';') = 0 then
  begin
    if (Length(Path) > 0) and (Path[Length(Path)] <> ';') then
      Path := Path + ';';
    Path := Path + InstallDir;
    RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
  end;
end;

