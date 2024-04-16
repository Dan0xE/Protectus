export interface SerializableSerialNumberData {
    state: string[];
    user_name: string;
    email: string;
    expire: string | null;
    max_build: string | null;
    running_time: string;
    user_data: string; // This will be a Base64 encoded string
}
