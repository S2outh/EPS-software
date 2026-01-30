import os


def test_flash(labgrid_session):
    t = labgrid_session
    d = t.get_driver("ProbeRsDriver", name="probe-eps")

    binary = "fw/eps-software"          # path if not in repo root

    print("flashing chip")
    # flash binary
    d.flash(binary)
    print("flashed chip")

    # attach with reset and read logs line-by-line from the test
    stream = d.attach_with_reset(binary, timeout=10.0)

    try:
        it = stream.iter_lines()

        try:
            first_line = next(it)
        except StopIteration:
            raise AssertionError("no output received from probe-rs")

        fw_version = os.environ.get("FW_VERSION", "no-version")
        fw_hash = os.environ.get("FW_HASH", "no-hash")

        assert f"Launching: FW version={fw_version} hash={fw_hash}" in first_line, f"unexpected first line: {first_line}"

    finally:
        # Always stop the subprocess, even on assertion failure
        stream.kill()
