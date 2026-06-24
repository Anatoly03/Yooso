/**
 * @file openapi-merge.ts
 * @author Anatoly Weinstein
 * @brief Merges multiple OpenAPI files into a single file.
 */

import * as fs from 'fs';

/** Get all arguments passed to the script. */
const args: string[] = process.argv.slice(2);

/** If has '-h' or '--help', print brief help message. */
if (args.includes('-h') || args.includes('--help')) {
    console.log('Usage: npm run openapi-merge <file1> <file2> ... <fileN>');
    console.log('Note: The `info` property of the first file will be used in the merged output.');
    process.exit(0);
}

/** Interface for OpenAPI files. */
interface OpenAPiFile {
    openapi?: string;
    info?: {
        name?: string;
        version?: string;
        authors?: string[];
    };
    paths?: any;
    components?: {
        schemas: any;
    };
}

/** Get all file JSON contents as a list. */
const fileContents: OpenAPiFile[] = await Promise.all(args.map(async (filePath) => {
    const fileData = await fs.promises.readFile(filePath, 'utf8');
    return JSON.parse(fileData);
}));

/** Validate openapi version matches. */
const openapiVersion = fileContents[0].openapi;
for (const fileContent of fileContents) {
    if (!fileContent.openapi) {
        console.error(`OpenAPI version missing in file: ${fileContent}`);
        console.log('Hint: Add `"openapi": "3.1.0"` to the file.');
        process.exit(1);
    }

    if (fileContent.openapi !== openapiVersion) {
        console.error(`OpenAPI version mismatch: ${fileContent.openapi} does not match ${openapiVersion}`);
        console.log('File 1: ', fileContents[0]);
        console.log('File 2: ', fileContent);
        process.exit(1);
    }
}

/** Create the resulting OpenAPI object. Data will be merged into this one. */
const merged: OpenAPiFile = {
    'openapi': openapiVersion,
    'info': fileContents[0].info,
    'paths': {},
    'components': {
        "schemas": {},
    },
};

for (const fileContent of fileContents) {
    // merge paths
    for (const [path, pathData] of Object.entries(fileContent.paths ?? [])) {
        merged['paths'][path] ??= {};

        for (const [method, methodData] of Object.entries(pathData as any)) {
            merged['paths'][path][method] = methodData;
        }
    }

    // merge components
    for (const [componentType, componentData] of Object.entries(
        fileContent.components?.schemas ?? [],
    )) {
        merged["components"]!["schemas"][componentType] ??= componentData;
    }

    // merge authors
    for (const author of (fileContent.info?.authors ?? [])) {
        if (!author) continue;
        if (merged.info!.authors!.includes(author)) continue;
        
        merged.info!.authors!.push(author);
    }
}

const outputString = JSON.stringify(merged, null, 2);
fs.writeFileSync('openapi.json', outputString, 'utf8');
console.log('Written to: openapi.json');
