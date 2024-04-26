import {invoke} from '@tauri-apps/api/core';
import {SerializableSerialNumberData} from './interface';
import {featureGate} from './helper';

//a more in-depth and more accurate documentation can be found here: https://vmpsoft.com/vmprotect/user-manual/

/** Gets the Users Hardware ID
 * @returns {Promise<string>} The Users Hardware ID
 */
export async function getHardwareId(): Promise<string | undefined> {
    return await featureGate('licensing', async () =>
        invoke('plugin:protectus|get_hwid_command')
    );
}

/** Sets the given Serial Number
 * @param {string} serialStr Serial Number to set
 * @returns {Promise<string>} Operation Result
 */
export async function setSerialNumber(
    serialStr: string
): Promise<string | undefined> {
    return await featureGate('licensing', async () =>
        invoke('plugin:protectus|set_serial_number_command', {
            serialStr,
        })
    );
}

/** Gets the Serial Number State
 * @returns {Promise<string>} Serial Number State
 */
export async function getSerialNumberState(): Promise<string | undefined> {
    return featureGate('licensing', async () =>
        invoke('plugin:protectus|get_serial_number_state_command')
    );
}

/** Gets the Serial Number Data
 * @returns {Promise<SerializableSerialNumberData>} Serialized Serial Number Data
 */
export async function getSerialNumberData(): Promise<
    SerializableSerialNumberData | undefined
> {
    const data: SerializableSerialNumberData | undefined = await featureGate(
        'licensing',
        async () => invoke('plugin:protectus|get_serial_number_data_command')
    );

    //we only check for undefined here
    if (data === undefined) throw new Error('No Serial Number Data returned!');

    if (data.user_data) {
        const decodedUserData = atob(data.user_data);
        data.user_data = decodedUserData;
    }
    return data;
}

/** Activates a given License Code
 * @param {string} code License Code To Activate
 * @returns {Promise<string>} Activation Result
 */
export async function activateLicense(
    code: string
): Promise<string | undefined> {
    return featureGate(
        'licensing',
        async () =>
            await invoke('plugin:protectus|activate_license_command', {code})
    );
}

/** Deactivates a given License Code
 *  @param {string} serial License Code to Deactivate
 *  @returns {Promise<string>} Operation Result
 */
export async function deactivateLicense(
    serial: string
): Promise<string | undefined> {
    return featureGate(
        'licensing',
        async () =>
            await invoke('plugin:protectus|deactivate_license_command', {
                serial,
            })
    );
}
