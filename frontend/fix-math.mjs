import fs from 'fs';
import path from 'path';

const contentDir = './content/docs';
const files = fs.readdirSync(contentDir).filter(f => f.endsWith('.mdx'));

for (const file of files) {
  const filePath = path.join(contentDir, file);
  let content = fs.readFileSync(filePath, 'utf-8');
  
  let lines = content.split('\n');
  let inMath = false;
  let newLines = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.trim() === '$$') {
      inMath = !inMath;
      newLines.push(line);
    } else {
      if (inMath && line.trim() === '') {
        // Skip empty lines in math block
      } else {
        // Revert the \{ back to { if I broke it inside math
        // But since I changed it, I'll just leave it or change back
        let fixedLine = line;
        if (inMath) {
            fixedLine = fixedLine.replace(/\\\{/g, '{').replace(/\\\}/g, '}');
        }
        newLines.push(fixedLine);
      }
    }
  }
  
  fs.writeFileSync(filePath, newLines.join('\n'));
}
console.log('Fixed math blocks');
