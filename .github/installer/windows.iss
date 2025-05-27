[Setup]
AppName=RDSLink
AppVersion={#AppVersion}
DefaultDirName={pf}\RDSLink
DefaultGroupName=RDSLink
OutputDir=.
OutputBaseFilename=RDSLinkSetup
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\rdslink.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\RDSLink"; Filename: "{app}\rdslink.exe"
