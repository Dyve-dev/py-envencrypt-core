# DPAPI module for Windows Data Protection API
import sys
from typing import TYPE_CHECKING

if sys.platform == "win32":
    from ..core import *  # type: ignore  # noqa: F403
elif TYPE_CHECKING:
    from ..core import *  # only for type hints, not for runtime  # noqa: F403

