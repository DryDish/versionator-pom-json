# versionator-pom-json

This binary is designed to take the version number from your package.json
and replace the version in your pom.xml file. Since pom.xml has multiple version
tags, pass the tag number you would like replaced. The app counts instances of
tags with the pattern '<version>'.

Usage:
      versionator [package.json path] [pom.xml path] [version tag number]

Example:
      versionator /path/to/package.json /path/to/pom.xml <number>
