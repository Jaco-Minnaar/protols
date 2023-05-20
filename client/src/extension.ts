import path from "path";
import * as vscode from "vscode";
import { ExtensionContext, workspace } from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    console.log("Congratulations, your extension is now active!");
    let lspBinary: string;
    // if (process.env["PROTOLS_DEBUG"]) {
    //     const filename = context.asAbsolutePath(
    //         path.join("server", "target", "debug", "protols_server")
    //     );
    //
    //     lspBinary = `${filename}${process.platform === "win32" ? ".exe" : ""}`;
    // } else {
    //     lspBinary = path.join(
    //         context.globalStorageUri.fsPath,
    //         "protols_server"
    //     );
    // }
    const filename = context.asAbsolutePath(
        path.join("server", "target", "debug", "protols_server")
    );

    lspBinary = `${filename}${process.platform === "win32" ? ".exe" : ""}`;

    const logLevel =
        vscode.workspace.getConfiguration("protols").get<string>("logLevel") ??
        "error";

    const serverOptions: ServerOptions = {
        run: { command: lspBinary, transport: TransportKind.stdio },
        debug: {
            command: lspBinary,
            transport: TransportKind.stdio,
            args: ["--log-level", logLevel],
        },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", pattern: "**/*.proto" }],
    };

    client = new LanguageClient(
        "protols",
        "Proto Language Server",
        serverOptions,
        clientOptions,
        true
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
