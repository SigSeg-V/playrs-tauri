import { invoke } from "@tauri-apps/api/tauri"
import { emit, listen } from '@tauri-apps/api/event'
import type { ClockTime } from "../Models/Player";

// Sends event to gst to play current queue
export async function playSound() {
  const unlisten = await invoke("play_sound");
  return unlisten;
}
  
  // Sends event to gst to pause current queue
export async function pauseSound() {
  const unlisten = await invoke("pause_sound");
  return unlisten;
}

  // Sends event to gst to pause current queue
export async function stopSound() {
  const unlisten = await invoke("stop_sound");
  return unlisten;
}

export function printClockTime(ct: ClockTime) {

  // pretty print 0 time
  if (ct.secs === 0) {
    return "--:--:--"
  }
  //                     Hours           Minutes     Seconds
  let time: any[] = [(ct.secs/60)/60, ct.secs/60, ct.secs];

  // get the leftover mins and secs from total duration
  time[1] %= 60;
  time[2] %= 60;
  
  // only need whole number precision
  time[0] = Math.floor(time[0]);
  time[1] = Math.floor(time[1]);
  time[2] = Math.round(time[2]);

  time.forEach(denom => {
    let formattedNum = denom.toLocaleString('en-US', {
      minimumIntegerDigits: 2,
      useGrouping: false
    });
    denom = formattedNum;
    time.push(formattedNum);
  });
  return time[0].concat(":", time[1], ":", time[2]);
}

export async function getDuration(duration: ClockTime) {
  console.log("duration button clicked");

  const unlisten = await listen<ClockTime>("get-duration", (event) => {
    
    duration = event.payload;
    console.log("inside await get-duration");
    console.log(event.payload);
    return duration;
  });
  return duration;
}

export async function getPosition(position: ClockTime) {
  console.log("position button clicked");

  const unlisten = await listen<ClockTime>("get-position", (event) => {
    
    position = event.payload;
    console.log("inside await get-position");
    console.log(event.payload);
  });
  return unlisten;
} 