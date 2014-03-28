# Rust 나이틀리 스냅샷 설치하기

(현재 공식 나이틀리 버전이 [제공되고 있습니다](/pages/install).
공식 버전을 사용하기 불편한 경우에만 사용해주세요.)

주의: 이 문서에서 언급하는 모든 나이틀리 스냅샷은 Rust 개발팀이 관여하지 않는 **비공식** 버전입니다.
이 버전을 사용해서 생기는 모든 문제에 대한 책임은 여러분에게 있습니다.
아직까지는 별로 그런 일은 없었지만, 혹시나 해서요.

## 우분투

> 팁: 리눅스를 사용하는 것 같은데 배포판을 잘 모르겠다면 먼저 `python -m platform`을 쳐서 중간에 `Ubuntu`가 나오는지 확인해 보세요.
> `python`이 없다고 나올 경우 `cat /etc/lsb-release`를 대신 해 보셔도 됩니다.

Hans Jørgen Hoel이 관리하는 [우분투용 Rust 나이틀리 스냅샷](https://launchpad.net/~hansjorg/+archive/rust)을 사용합니다.
터미널에서 다음 명령을 입력하고 지시를 따르면 설치됩니다.
관리자 모드(`sudo`)를 쓰기 위해 로그인된 계정의 암호를 입력해야 할 수 있습니다.

    $ sudo add-apt-repository ppa:hansjorg/rust
    $ sudo apt-get update
    $ sudo apt-get install rust-nightly

혹은 `rust-0.8`, `rust-0.9` 등을 대신 설치할 수도 있습니다.

나중에 업데이트를 하려면 마지막 두 명령만 되풀이하면 됩니다.
(혹은 `sudo apt-get upgrade`로 Rust를 포함한 모든 패키지를 업데이트할 수 있습니다.)

    $ sudo apt-get update
    $ sudo apt-get install rust-nightly

> 팁: 여러 개의 `rustc`가 설치되었을 경우 각 릴리스는 `/usr/lib/rust-<버전>/bin/rustc`에 들어가고
> 마지막으로 설치되거나 업데이트된 버전만 `rustc`에 심볼릭 링크됩니다.
> 심볼릭 링크를 바꾸려면 우분투의 링크 관리 기능을 사용합니다.
>
>     $ sudo update-alternatives --config rustc
>     대체 항목 rustc에 대해 (/usr/bin/rustc 제공) 3개 선택이 있습니다.
>
>       선택       경로                                우선순  상태
>     ------------------------------------------------------------
>     * 0            /usr/lib/rust/rust-nightly/bin/rustc   150       자동 모드
>       1            /usr/lib/rust/rust-0.8/bin/rustc       80        수동 모드
>       2            /usr/lib/rust/rust-0.9/bin/rustc       90        수동 모드
>       3            /usr/lib/rust/rust-nightly/bin/rustc   150       수동 모드
>
>     기본 사항[*]을 사용하려면 엔터, 다른 것을 사용하려면 번호를 입력하십시오:

> 팁: 데비안에서도 libc 버전이 호환되면 우분투 패키지를 사용할 수 있습니다.

## 윈도

[MinGW 인스톨러](http://sourceforge.net/projects/mingw/files/latest/download?source=files)를 받아서 실행한 뒤 지시에 따릅니다.
그래픽 패키지 매니저를 설치할지 선택하는 칸이 있는데 선택해도 되지만 여기서는 사용하지 않습니다.

> 팁: 그래픽 패키지 매니저를 실수로 안 설치했을 경우 `mingw-get install mingw-get-gui` 명령을 사용해서 나중에 설치할 수 있습니다.

MinGW는 기본적으로 `PATH` 환경 변수에 추가되지 않습니다.
`rustc`를 아무데서나 실행하려면 이 환경 변수가 필요하므로, MinGW 실행파일이 위치한 경로를 `PATH`에 추가해 줍니다.
(기본값대로 설치했다면 `C:\MinGW\bin`을 뒤에 넣으면 됩니다.)
제어판의 [시스템] → \[고급 시스템 설정] (또는 [고급]) → [환경 변수]에서 설정할 수 있습니다.

명령 프롬포트를 열고 다음을 입력합니다.

    > mingw-get install mingw32-base

> 팁: 이 명령은 C 컴파일러도 함께 설치합니다. C 컴파일이 필요 없다면 `mingw-get install gcc`만 해도 됩니다.

다음으로, [Chocolatey](http://chocolatey.org/)를 설치합니다.
홈페이지에 나와 있는 명령을 복사해서 명령 프롬포트에 붙여 넣으세요.
(명령 프롬포트 창의 오른쪽 위 아이콘을 눌러서 [편집] → [붙여넣기]를 선택합니다.)

    > @powershell -NoProfile -ExecutionPolicy unrestricted -Command "iex ((new-object net.webclient).DownloadString('https://chocolatey.org/install.ps1'))" && SET PATH=%PATH%;%systemdrive%\chocolatey\bin

이제 Chocolatey를 사용하여 Rust를 설치합니다.
Heather가 관리하는 [NuGet Rust 패키지](https://www.nuget.org/packages/Rust/)를 사용합니다.
패키지가 120MB 정도에 달하기 때문에 다운로드에 다소 시간이 걸릴 수 있습니다.

    > cinst Rust

> 팁: cinst를 실행하고 얼마 안 지나 "Unable to read package from path" 오류가 날 경우,
> 기존 NuGet 캐시와 충돌이 나서 패키지가 덜 받아졌을 경우가 대부분입니다.
> `%AppData%\Local\NuGet\Cache`에서 `Rust`로 시작하는 파일을 삭제하고 다시 시도하세요.

설치가 종료되면 `C:\Chocolatey\lib\Rust.<버전>\bin`에 `rustc.exe` 등이 설치됩니다.
명령 프롬포트에서 `rustc -v`를 입력해서 올바른 버전이 설치되었는지 확인해 보세요.

나중에 업데이트를 하려면 Chocolatey의 업데이트 기능을 이용합니다.

    > cup Rust

### MSYS와 함께 사용하기

MinGW에는 유닉스 셸을 흉내내는 MSYS라는 서브시스템이 있습니다.
이 서브시스템은 Rust 컴파일러를 포함해서 `make` 같은 유닉스 빌드 시스템을 쓰는 프로젝트를 위해 필요합니다.

MSYS를 설치하려면 다음 명령을 실행합니다.

    > mingw-get install msys-base

이제 MinGW 디렉토리 안의 `msys\1.0\msys.bat`를 실행하면 MSYS 셸이 뜹니다.

> 팁: MSYS를 설치해도 자동으로 바로가기가 만들어지진 않습니다. `msys.bat`에 대한 바로가기를 만드는 걸 추천합니다.

> 팁: 만약 MinGW를 위에 제시된 방법으로 설치해 쓰지 않는다면, 특히 `PATH`에 MinGW가 없다면,
> 셸 안에서 `/postinstall/pi.sh`를 실행해서 지시에 따르셔야 합니다. (보통은 필요 없습니다.)

Chocolatey로 설치한 `rustc`는 MSYS에서도 큰 문제 없이 동작합니다만,
Chocolatey가 배치 파일을 사용하는데 MSYS 셸이 (현재는) 배치 파일을 지원하지 않아서 바로 실행할 수 없습니다.
따라서 MinGW 디렉토리 안에 다음과 같은 파일을 따로 만들어 주어야 합니다.

* `msys\1.0\bin\rustc`
* `msys\1.0\bin\rustdoc`
* `msys\1.0\bin\rustpkg`

각 파일에는 다음과 같은 내용을 집어 넣으면 됩니다.

    #!/bin/sh
    cmd //c $(basename "$0") "$@"

이렇게 하면 MSYS에서도 `rustc` 등의 명령을 사용할 수 있습니다.

