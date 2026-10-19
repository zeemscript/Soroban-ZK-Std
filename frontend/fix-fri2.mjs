import fs from 'fs';

let content = fs.readFileSync('content/docs/fri.mdx', 'utf-8');
content = content.replace(/\\\{x_0,x_1,\\ldots,x_\{N-1\}\\}/g, '[x_0,x_1,\\ldots,x_{N-1}]');
content = content.replace(/\\\{x\^2 ;\|; x\\in D_i\\\}/g, '[x^2 ;|; x\\in D_i]');
// Also replace if they are unescaped
content = content.replace(/\{x_0,x_1,\\ldots,x_\{N-1\}\}/g, '[x_0,x_1,\\ldots,x_{N-1}]');
content = content.replace(/\{x\^2 ;\|; x\\in D_i\}/g, '[x^2 ;|; x\\in D_i]');

fs.writeFileSync('content/docs/fri.mdx', content);
console.log('Fixed fri.mdx set brackets');
