import * as fs from 'node:fs/promises';
import { compileFromFile } from 'json-schema-to-typescript'

// compile from file
compileFromFile('src/review.json')
  .then(ts => fs.writeFile('types/NewReview.d.ts', ts))

