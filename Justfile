publish:
    #!/bin/bash
    cargo login
    cargo publish --dry-run