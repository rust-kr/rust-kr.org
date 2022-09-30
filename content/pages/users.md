+++
+++

## 외국의 러스트 사용례

아래 정보는 2022년 9월 기준입니다. 링크한 글에는 연도를 병기하였습니다.

### 구글

구글([Google](https://en.wikipedia.org/wiki/Google), [GOOG](https://finance.yahoo.com/quote/GOOG))은 세계에서 3번째로 큰 회사입니다. 구글은 [안드로이드](https://source.android.com/) 모바일 운영 체제 개발에 러스트를 사용하고 있습니다. 구글 보안 블로그의 글 [Rust in the Android platform (2021)](https://security.googleblog.com/2021/04/rust-in-android-platform.html)을 참고하세요. 기술적인 측면에 대해서는 같은 블로그의 글 [Integrating Rust Into the Android Open Source Project (2021)](https://security.googleblog.com/2021/05/integrating-rust-into-android-open.html)를 참고하세요.

구글은 러스트 재단의 [창립 회원]입니다. 구글은 러스트와 C++을 함께 사용하는데 많은 관심을 가지고 있으며, 이를 위해 [autocxx](https://github.com/google/autocxx)를 개발하고 있습니다. 구글은 러스트와 [Bazel](https://bazel.build/) 빌드 시스템을 함께 사용하는데 많은 관심을 가지고 있으며, 이를 위해 [cargo-raze](https://github.com/google/cargo-raze)를 개발하고 있습니다.

### 아마존

아마존([Amazon](https://en.wikipedia.org/wiki/Amazon_(company)), [AMZN](https://finance.yahoo.com/quote/AMZN))은 세계에서 4번째로 큰 회사입니다. 아마존은 [Firecracker](https://firecracker-microvm.github.io/) 마이크로 VM을 러스트로 개발하였습니다. Firecracker는 AWS의 서버리스 컴퓨팅 서비스인 [AWS Lambda](https://aws.amazon.com/lambda/)를 실행하는데 쓰이고 있습니다. AWS 블로그의 글 [Firecracker: Lightweight Virtualization for Serverless Computing (2018)](https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/)을 참고하세요.

아마존 AWS는 러스트 재단의 [창립 회원]입니다. 아마존 AWS는 러스트와 러스트를 위한 비동기 런타임인 [Tokio](https://tokio.rs/)의 개발자들을 고용하고 있습니다. AWS 오픈소스 블로그의 글 [Why AWS loves Rust, and how we'd like to help (2020)](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)를 참고하세요.

### 페이스북

페이스북([Facebook](https://en.wikipedia.org/wiki/Facebook), [FB](https://finance.yahoo.com/quote/FB))은 세계에서 가장 큰 SNS입니다. 페이스북은 서버 개발에 러스트를 사용하고 있습니다. 페이스북에서 러스트가 사용된 역사에 대해서는 페이스북 기술 블로그의 글 [A brief history of Rust at Facebook (2021)](https://engineering.fb.com/2021/04/29/developer-tools/rust/)을, 러스트가 C++, [Hack](https://hacklang.org/), 파이썬에 이어 페이스북 서버 팀의 네번째 공식 언어가 된 이야기에 대해서는 같은 블로그의 글 [Programming languages endorsed for server-side use at Meta (2022)](https://engineering.fb.com/2022/07/27/developer-tools/programming-languages-endorsed-for-server-side-use-at-meta/)를 참고하세요.

페이스북은 러스트 재단의 [플래티넘 회원]입니다.

### 모질라

모질라([Mozilla](https://en.wikipedia.org/wiki/Mozilla))는 수억명이 사용하는 웹 브라우저인 [파이어폭스](https://www.mozilla.org/firefox/browsers/)를 개발하고 있습니다. 파이어폭스는 러스트로 스타일링과 렌더링을 재작성하였으며, 러스트의 장점을 살려 병렬화를 해냈습니다. 모질라 기술 블로그의 글 [Inside a super fast CSS engine: Quantum CSS (aka Stylo) (2017)](https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo/)와 [The whole web at maximum FPS: How WebRender gets rid of jank (2017)](https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)를 참고하세요.

파이어폭스의 스타일링은 여전히 웹 브라우저들 중에서 가장 빠르며, 다른 웹 브라우저들은 5년이 지났는데도 파이어폭스의 성능을 따라잡지 못하고 있습니다. Nolan Lawson의 블로그의 글 [Style scoping versus shadow DOM: which is fastest? (2022)](https://nolanlawson.com/2022/06/22/style-scoping-versus-shadow-dom-which-is-fastest/)는 다음과 같이 말하고 있습니다.

> Firefox's Stylo engine is fast. Like, really fast. Like, so fast that, if every browser were like Firefox, there would be little point in discussing style performance.

모질라는 러스트 재단의 [창립 회원]입니다.

### 드롭박스

드롭박스([Dropbox](https://en.wikipedia.org/wiki/Dropbox), [DBX](https://finance.yahoo.com/quote/DBX))는 널리 쓰이는 파일 호스팅 서비스입니다. 드롭박스는 수만대의 서버에 엑사바이트가 넘는 파일을 저장하는데 러스트를 사용하고 있습니다. 드롭박스 기술 블로그의 글 [Scaling to exabytes and beyond (2016)](https://dropbox.tech/infrastructure/magic-pocket-infrastructure)를 참고하세요. 기술적인 측면에 대해서는 같은 블로그의 글 [Inside the Magic Pocket (2016)](https://dropbox.tech/infrastructure/inside-the-magic-pocket)을 참고하세요. 러스트가 어떻게 쓰였는지에 대해서는 QCon San Francisco 2016의 발표 [Go-ing Rust: Optimizing Storage at Dropbox (2016)](https://qconsf.com/sf2016/sf2016/presentation/going-rust-optimizing-storage-dropbox.html)를 참고하세요.

드롭박스는 수억명이 사용하는 클라이언트에서도 파일을 동기화하는데 러스트를 사용하고 있습니다. 드롭박스 기술 블로그의 글 [Rewriting the heart of our sync engine (2020)](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine)을 참고하세요.

드롭박스는 러스트 재단의 [실버 회원]입니다.

[창립 회원]: https://foundation.rust-lang.org/members/
[플래티넘 회원]: https://foundation.rust-lang.org/members/
[실버 회원]: https://foundation.rust-lang.org/members/
