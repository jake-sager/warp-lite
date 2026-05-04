#!/usr/bin/env python3
"""Warp Lite does not generate font fallback metadata from GCS.

Fallback font data should be checked in or generated from local assets only.
"""

raise SystemExit("GCS-backed font fallback generation is removed in warp-lite.")
