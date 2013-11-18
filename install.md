Rust는 현재 언어와 표준 라이브러리가 계속하여 발전하고 있으며,
따라서 Rust의 현재 상태를 경험하기 위해서는 릴리즈 버전이 아닌 최신 버전을 추천합니다.

# 직접 빌드하기

[github 저장소][rust-github]에서 소스를 받은 다음,
`./configure`가 문제 없이 실행되면 `make`를 실행하여 빌드해주세요.
빌드가 완료되면 `make install DESTDIR=<경로>`로 설치할 수 있습니다.
마지막으로, rust가 설치된 경로의 `bin/` 디렉토리를 `PATH`에 추가해주세요.

처음 빌드할 때에는 [LLVM][llvm]이 먼저 빌드되는데, 여기에서 상당한 시간이 소모됩니다.
일단 한번 llvm이 빌드되면 llvm이 업그레이드 되지 않는 한 기존 빌드가 계속 사용되기 때문에
같은 저장소에서 지속적으로 빌드하는 것이 좋습니다.

## 윈도

윈도에서 Rust를 빌드하려면 먼저 [mingw][]를 준비해야 합니다.

-   [mingw 다운로드 페이지][mingw-sf-files]에서 인스톨러를 받아서 실행해주세요.
    몇 가지 기본 패키지가 받아진 다음 MinGW Installation Manager가 나타납니다.
-   "mingw-developer-toolkit", "mingw32-base", "mingw32-gcc-g++"를 선택해 설치해주세요.
    mingw 디렉토리에서 `msys/1.0/msys.bat`을 실행하면 셸이 뜹니다.
-   `/postinstall/pi.sh`를 실행해주세요.
    몇 가지 세팅이 끝나면 Rust를 빌드할 수 있는 환경이 준비됩니다.

# 패키지

몇몇 배포판에서는 공식/비공식적인 패키지를 제공합니다.

-   우분투용 비공식 [나이틀리 패키지][rust-nightly-ubuntu]
-   아크 리눅스용 비공식 [나이틀리 패키지][rust-nightly-arch]

[러스트 위키 페이지][rust-packages]에도 관련 정보를 찾을 수 있습니다.

(Homebrew의 경우 바이너리가 아니라 소스 빌드이며, 매번 llvm을 새로 빌드할 수 있어
추천하지 않습니다.)

[rust-github]: http://github.com/mozilla/rust
[rust-nightly-ubuntu]: https://launchpad.net/~hansjorg/+archive/rust
[llvm]: http://llvm.org/
[mingw]: http://mingw.org/
[mingw-sf-files]: http://sourceforge.net/projects/mingw/files/
[rust-packages]: https://github.com/mozilla/rust/wiki/Doc-packages,-editors,-and-other-tools
[rust-nightly-arch]: http://pkgbuild.com/~thestinger/repo/
