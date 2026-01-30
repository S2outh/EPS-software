import os
import pty
import select
import subprocess
import time
from typing import Optional, Iterator

import attr
from labgrid.driver.common import Driver
from labgrid.factory import target_factory
from labgrid.resource import NetworkService

class ProcessLogStream:
    def __init__(self, proc: subprocess.Popen[str], timeout_s: Optional[float]):
        self._proc = proc
        self._timeout_s = timeout_s
        self._start = time.monotonic()

    @property
    def pid(self) -> int:
        return self._proc.pid

    def terminate(self) -> None:
        if self._proc.poll() is None:
            self._proc.terminate()

    def kill(self) -> None:
        if self._proc.poll() is None:
            self._proc.kill()

    def wait(self, timeout_s: Optional[float] = None) -> int:
        return self._proc.wait(timeout=timeout_s)

    def iter_lines(self) -> Iterator[str]:
        """
        Yields merged stdout/stderr lines as they arrive.

        Raises:
          TimeoutError if timeout_s was set and exceeded.
        """
        assert self._proc.stdout is not None

        for raw in iter(self._proc.stdout.readline, ""):
            if self._timeout_s is not None and (time.monotonic() - self._start) > self._timeout_s:
                raise TimeoutError(f"process timed out after {self._timeout_s}s")

            yield raw.rstrip("\n")

        # Process ended; if it ended with an error, surface it.
        rc = self._proc.wait(timeout=1)
        if rc != 0:
            raise subprocess.CalledProcessError(rc, self._proc.args)

@target_factory.reg_driver
@attr.s(eq=False)
class ProbeRsDriver(Driver):
    bindings = {"svc": NetworkService}

    token = attr.ib(default="")
    chip = attr.ib(default="")
    ws_scheme = attr.ib(default="ws")  # "ws" or "wss"
    probe_rs_bin = attr.ib(default="probe-rs")

    def _base_args(self):
        host = f"{self.ws_scheme}://{self.svc.address}:{self.svc.port}"
        return [self.probe_rs_bin, "--host", host, "--token", self.token]

    def list_probes(self):
        return subprocess.check_output(self._base_args() + ["list"], text=True)

    def flash(self, path: str):
        cmd = self._base_args() + ["download", "--chip", self.chip, path]

        output = subprocess.run(
            cmd,
            text=True,
            capture_output=True,
            timeout=20,
            check=True,
        )

        print(output.stdout)
        print(output.stderr)

    def reset(self):
        subprocess.check_call(self._base_args() + ["reset", "--chip", self.chip])

    def attach_with_reset(self, path: str, timeout: Optional[float] = 10.0,):
        proc = subprocess.Popen(
            self._base_args() + ["attach", "--chip", self.chip, path, "--connect-under-reset"],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
        )
        return ProcessLogStream(proc, timeout)
