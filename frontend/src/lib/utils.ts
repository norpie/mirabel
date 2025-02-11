import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
const charsLength = chars.length;

export function generateId(): string {
    let result = "";
    for (let counter = 0; counter < 20; counter++) {
        result += chars.charAt(Math.floor(Math.random() * charsLength));
    }
    return result;
}
