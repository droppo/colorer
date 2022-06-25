%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: colorer
Summary: Simple command line utility that add color to commands that do not have it by default.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/droppo/colorer

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/etc/colorer/df.toml
/etc/colorer/dig.toml
/etc/colorer/docker.toml
/etc/colorer/env.toml
/etc/colorer/free.toml
/etc/colorer/lastb.toml
/etc/colorer/last.toml
/etc/colorer/lsns.toml
/etc/colorer/ls.toml
/etc/colorer/nmap.toml
/etc/colorer/nslookup.toml
/etc/colorer/ping.toml
