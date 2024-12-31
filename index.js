#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Get directory where package is installed
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Show help if requested
if (process.argv.includes('--help')) {
    console.log();
    console.log("Play text art animations in the terminal\n");
    console.log("Usage: npx firew0rks [folder] [loops]");
    console.log("\t[folder]\tFolder containing text art frames (default: fireworks)");
    console.log("\t[loops]\t\tNumber of times to loop the animation or use -1 to loop until the user terminates the program (default: 20)");
    console.log();
    process.exit(0);
}

// Use default values if no arguments provided
const folderName = process.argv[2] || 'fireworks';
const loops = process.argv[3] ? parseInt(process.argv[3]) : 20;

// Resolve folder path relative to package location
const folderPath = path.join(__dirname, folderName);

if (!fs.existsSync(folderPath)) {
    console.log(folderName + " could not be found");
    process.exit(0);
}

const textFiles = [];
let numFound = 0;
let filesExist = true;

while (filesExist) {
    const fileName = path.join(folderPath, numFound + ".txt");
    
    if (fs.existsSync(fileName)) {
        textFiles.push(fs.readFileSync(fileName, 'utf8'));
        numFound++;
    } else {
        filesExist = false;
    }
}

if (textFiles.length === 0) {
    console.log(folderName + " did not have text art files");
    process.exit(0);
}

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

async function playAnimation() {
    let i = 0;
    let first = true;
    // Calculate number of lines to move up based on first frame
    const backspaceAdjust = '\x1b[A'.repeat(textFiles[0].split('\n').length + 1);

    while (i < loops || loops === -1) {
        for (const frame of textFiles) {
            if (!first) {
                process.stdout.write(backspaceAdjust);
            }
            
            process.stdout.write(frame + '\n');
            
            first = false;
            await sleep(50); // 0.05 seconds
        }
        i++;
    }
}

playAnimation();