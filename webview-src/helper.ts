import {invoke} from '@tauri-apps/api/core';

interface Features {
    licensing: boolean;
    service: boolean;
}

type GenericFunc<F> = () => Promise<F>;

/** Checks if the required feature for the passed function is enabled on the rust backend
 * @param {keyof Features} featureEnabled Feature that has to be enabled for this function to be executed
 * @param {GenericFunc<F>} functionToExecute Functions that should be executed if the required feature is enabled
 * @throws Throws if the required feature is not enabled in the rust backend
 */
export async function featureGate<F>(
    featureEnabled: keyof Features,
    functionToExecute: GenericFunc<F>
): Promise<F | undefined> {
    const features: Features = await invoke(
        'plugin:protectus|feature_check_command'
    );

    if (features[featureEnabled]) {
        return await functionToExecute();
    }

    // NOTE functionsToExecute will look something like this:
    // async () => invoke('plugin:protectus|get_hwid_command')
    // We only care about what's gets passed inside of the invoke(...) function
    // *after* the 'plugin:protectus|' part.
    //
    // Brace your eyes for some regex magic:
    const extractCommandName = (fn: GenericFunc<F>) =>
        fn.toString().match(/invoke\('plugin:protectus\|([^']+)/)?.[1] ??
        'unknown_function';

    const functionName = extractCommandName(functionToExecute);

    throw new Error(
        `${functionName} called but required feature ${featureEnabled} is not enabled!`
    );
}
