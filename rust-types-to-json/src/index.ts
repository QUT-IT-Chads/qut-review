import * as fs from 'node:fs/promises';
import { compileFromFile } from 'json-schema-to-typescript';

// Convert schema to TS types
async function compileAll(dir: string, filename: string) {
    compileFromFile(`${dir}/${filename}.json`)
        .then(ts => fs.writeFile(`out/types/${filename}.ts`, ts))
}

// Find all schemas in the schemas directory
async function main() {
    await fs.mkdir('out/types', { recursive: true });

    const dir = 'src/schemas';
    const filenames = await fs.readdir(dir);
    filenames
        .filter(filename => filename.endsWith('.json'))
        .forEach(filename => compileAll(dir, filename.slice(0, -5)));
}

main();
