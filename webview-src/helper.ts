import {invoke} from '@tauri-apps/api/core';

interface Features {
    licensing: boolean;
    service: boolean;
}

type GenericFunc<F> = () => Promise<F>;

/** Checks if the required feature for the passed function is enabled on the rust backend
 * @param {keyof Features} featureEnabled Feature that has to be enabled for this function to be executed
 * @param {GenericFunc<F>[]} functionsToExecute Functions that should be executed if the required feature is enabled
 * @throws Throws if the required feature is not enabled in the rust backend
 */
export async function featureGate<F>(
    featureEnabled: keyof Features,
    ...functionsToExecute: GenericFunc<F>[]
): Promise<F | undefined> {
    const features: Features = await invoke(
        'plugin:protectus|feature_check_command'
    );

    if (features[featureEnabled]) {
        let lastResult: F | undefined = undefined;
        for (const fn of functionsToExecute) {
            lastResult = await fn();
        }
        return lastResult;
    }

    throw new Error(
        `${functionsToExecute} called but required feature ${featureEnabled} is not enabled!`
    );
}
