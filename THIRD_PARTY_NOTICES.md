# Third-party notices

## First-party license

RelayForge's first-party material is licensed under the PolyForm Noncommercial
License 1.0.0 in [`LICENSE`](LICENSE). That license contains the required notice
`Copyright (c) 2026 special-place-administrator`.
RelayForge's license does not relicense imported template material or
third-party materials; those remain governed by their respective original terms
and notices.

## Imported project template

RelayForge was initialized from
[jequihua/agentic-project-template-v4](https://github.com/jequihua/agentic-project-template-v4)
at revision [`169a74c66140743008bef374f2c7d0389ee81cb9`](https://github.com/jequihua/agentic-project-template-v4/tree/169a74c66140743008bef374f2c7d0389ee81cb9),
as recorded in D001. At that pinned revision, the GitHub source tree contained no
standalone file named LICENSE, LICENCE, COPYING, or NOTICE. A bounded search of
the pinned `README.md` and `pyproject.toml` for license, copyright, permission,
grant, proprietary, commercial, and rights terms returned no matches. These are
observations about the checked paths only; they do not establish permission or
determine the licensing status of imported template material. No template
license text or reuse grant is asserted here.

## Python runtime and build requirements

The following covers only the Python dependencies currently declared in
`pyproject.toml`. License texts are retained under [`licenses/`](licenses/).

| Package | Declared role and version evidence | License evidence |
| --- | --- | --- |
| PyYAML | Runtime requirement `>=6.0.3,<7`; version 6.0.3 was observed in the inspected Python environment. The project requirement is not pinned. | MIT; original package license retained in [`licenses/PyYAML-LICENSE.txt`](licenses/PyYAML-LICENSE.txt). Source: [PyYAML 6.0.3](https://github.com/yaml/pyyaml/tree/6.0.3). |
| setuptools | Build requirement `>=68`; version 80.9.0 was observed in the inspected Python environment. The project requirement is not pinned. | MIT; original package license retained in [`licenses/setuptools-LICENSE.txt`](licenses/setuptools-LICENSE.txt). Source: [setuptools 80.9.0](https://github.com/pypa/setuptools/tree/v80.9.0). |
| wheel | Build requirement with no version constraint. No installed wheel distribution was available in the inspected Python environment. | The upstream repository identifies its license as MIT; the license text was retained in [`licenses/wheel-LICENSE.txt`](licenses/wheel-LICENSE.txt) from official commit `4ae6d07d38611cb0619864621c06fc032622eaef`. This does not identify a resolved wheel version. Source: [wheel LICENSE.txt at that commit](https://github.com/pypa/wheel/blob/4ae6d07d38611cb0619864621c06fc032622eaef/LICENSE.txt). |

This notice is not a comprehensive inventory of future Rust dependencies or
other materials that may be added later.
