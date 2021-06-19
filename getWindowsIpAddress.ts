import { readLines } from "https://deno.land/std@0.99.0/io/mod.ts";

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

console.log(await getWindowsIpAddress());
