Rust 설치하기
========

### A. [Rustup] 사용하기
<b>[rustup]</b>을 쓰면 윈도우, 맥, 리눅스 등 대부분의 환경에서 별도의 지식 없이
손쉽게 러스트를 설치할 수 있습니다. [Rust 공식 설치 가이드]가 제일 권장하는
방법이므로, 안심하고 사용하셔도 됩니다.

리눅스와 맥의 경우 셸에서 아래의 커맨드 한줄만 입력하면 Rust 컴파일러와
패키지매니저가 한번에 설치됩니다.

```bash
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

윈도우의 경우, [rustup‑init.exe] 파일을 받아서 실행하기만 하면 시스템에 Rust가
설치됩니다.

&nbsp;

### B. 패키지매니저 사용하기
[rustup]을 쓰기 싫은경우, 각 시스템에 기본으로 탑재되어있는 패키지매니저를
사용하여 설치해도 됩니다. 그러나 이와 같은 방식으로 설치할경우, **상당히 늦은
버전의 Rust가 설치되므로 주의하세요**.

```bash
# Ubuntu, Debian
apt install rustc cargo
# Fedora
dnf install rust

# macOS에서 Homebrew를 쓰는 경우
brew install rust
# macOS에서 Mac Ports를 쓰는 경우
port install rust

# 윈도우즈에서 Chocolatey를 쓰는 경우
choco install rust     # GNU ABI
choco install rust-ms  # MSVC ABI

# OpenSUSE
zypper in rust
# Gentoo Linux
emerge dev-lang/rust
# Arch Linux, Manjaro Linux, msys2
pacman -S rust
# Alpine Linux
apk add rust

# FreeBSD
pkg install lang/rust
# OpenBSD
pkg_add rust
```

<br>

### C. 직접 러스트를 빌드하기
일반 사용자의 경우, 아주 특이한 경우를 제외하면 Rust를 직접 빌드할 필요가 전혀
없습니다. 하지만 Rust 컴파일러를 직접 고치고 싶으시다면, 개발환경에서 Rust를
빌드하셔야합니다.

1.  개발환경에 아래의 디펜던시들이 모두 있는지 먼저 체크합니다.

    * `g++` 4.7 or later or `clang++` 3.x
    * `python` 2.7 (but not 3.x)
    * GNU `make` 3.81 or later
    * `cmake` 3.4.3 or later
    * `curl`
    * `git`

1.  [Github 저장소]에서 소스를 다운 받으세요.

    ```bash
    git clone https://github.com/rust-lang/rust.git
    cd rust/
    ```

1.  해당 디렉토리로 이동해 아래와 같이 빌드를 수행해주세요.

    ```bash
    ./x.py build
    ```

1.  빌드가 끝났다면 두가지의 선택지가 있습니다.

    1.  **추천!** 빌드된 현재의 저장소를 그대로 사용하기.

        이 경우는 rust 가 저장소 밖을 벗어나지 않아 다른곳을 더럽히지 않는다는
        장점이 있습니다. Build 가 완료되면 생성되는 디렉토리속의 'stage2/bin/'
        를 그냥 사용하시면 됩니다.

    1.  `make install DESTDIR=<경로>` 를 이용해 특정한 경로에 rust 를 설치하기.

        이 경우에는 원하는 장소에 rust 를 설치하여 사용할 수 있습니다.

        ```bash
        make install DESTDIR="/Users/Jeyraof/bin/"
        ```

1.  위에서 결정한 바이너리의 디렉토리를 PATH 에 등록해 줍니다.

    ```bash
    export PATH=$PATH:<경로>
    ```

    매번 수행하기 귀찮다면 자신의 profile 에 등록해 놓으셔도 됩니다.

    `~/.profile` 또는 `~/.bash_profile` 의 내용:

    ```bash
    export PATH=$PATH:<경로>
    ```

    이제 새로운 세션부터는 그냥 rustc, rustpkg, rustdoc 을 사용할 수 있습니다.

    4.2의 "빌드된 현재의 저장소를 그대로 사용하기" 를 사용하시려면 다음과 같이
    하시면 됩니다. (<경로>는 보통 '<소스디렉토리>/x86_64-unknown-linux-gnu' 이런
    형식으로 생성됩니다)

    ```bash
    export PATH=$PATH:<경로>/stage2/bin
    export LD_LIBRARY_PATH=$:<경로>/stage2/lib
    ```

    ~/.profile 또는 ~/.bash_profile 의 내용:

    ```bash
    export PATH=$PATH:<경로>/stage2/bin
    export LD_LIBRARY_PATH=$PATH:<경로>/stage2/lib
    ```

1.  추가 정보

    처음으로 빌드할 때에는 [LLVM]이 먼저 빌드됩니다. 여기에서 상당한 시간이
    소모되는데, 일단 한번 빌드가 되고나면 llvm 자체가 업그레이드 되지 않는 한
    **기존 빌드결과물이 계속해서 사용**됩니다. 그러므로 같은 저장소에서
    지속적으로 빌드하는것이 좋습니다.

<br>

### D. 직접 러스트를 빌드하기 (Windows의 경우)
윈도우에서 Rust를 빌드하려면 먼저 [mingw]를 준비해야 합니다.

-   [mingw 다운로드 페이지]에서 인스톨러를 받아서 실행해주세요. 몇 가지 기본
    패키지가 받아진 다음 MinGW Installation Manager가 나타납니다.
-   "mingw-developer-toolkit", "mingw32-base", "mingw32-gcc-g++"를 선택해
    설치해주세요. mingw 디렉토리에서 `msys/1.0/msys.bat`을 실행하면 셸이 뜹니다.
-   `/postinstall/pi.sh`를 실행해주세요. 몇 가지 세팅이 끝나면 Rust를 빌드할 수
    있는 환경이 준비됩니다.

[Rust 공식 설치 가이드]: https://www.rust-lang.org/ko-KR/install.html
[rustup]: https://rustup.rs
[rustup‑init.exe]: https://win.rustup.rs/
[Github 저장소]: https://github.com/rust-lang/rust
[LLVM]: http://llvm.org/
[mingw]: http://mingw.org/
[mingw 다운로드 페이지]: http://sourceforge.net/projects/mingw/files/
