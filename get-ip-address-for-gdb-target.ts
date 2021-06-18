import { readLines } from "https://deno.land/std@0.99.0/io/mod.ts";
import * as path from "https://deno.land/std@0.99.0/path/mod.ts";

const repoLocation = '/home/pfes/discovery';

async function getWindowsIpAddress() {
  const fileReader = await Deno.open("/etc/resolv.conf");

  for await (const line of readLines(fileReader)) {
    const match = line.match(/^nameserver (\d+\.\d+\.\d+\.\d+)$/);
    if (match) {
      return match[1];
    }
  }

  throw new Error("Windows IP address not found");
}

await Deno.open(path.join(repoLocation, 'src/openocd.gdb'));
// write over file
