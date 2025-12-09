import os
import sys
import json
import hashlib
import os
DEBUG = True
CACHE_FOLDER = "__local_cache__"
def local_cacher(prefix:str="LOCAL_CACHE", cache_folder:str=CACHE_FOLDER):
    if not os.path.exists(cache_folder):
        os.mkdir(cache_folder)

    def _local_cacher_deco(func):
        def wrapper(_skip_self, **kwargs):
            params = kwargs
            param_key = json.dumps(params)
            ### Params must be hashable
            hash = hashlib.sha256(param_key.encode('utf-8')).hexdigest()
            store_key = f"{prefix}-" + hash
            cache_path = cache_folder + "/" + store_key + ".json"

            try:
                if os.path.exists(cache_path):
                    if DEBUG:
                        print("[DEBUG] Local cache hit: ", store_key)
                    with open(cache_path, "r") as f:
                        return json.load(f)["ret_val"]
                else:
                    if DEBUG:
                        print("[DEBUG] Local cache miss: ", store_key)
                    ret_val = func(_skip_self, **kwargs)
                    with open(cache_path, "w") as f:
                        json.dump({"ret_val": ret_val}, f)
                    return ret_val
            except Exception as e:
                print("[ERROR] Local cache failed to set key: ", store_key, e)

            return "ERROR"
        return wrapper
    return _local_cacher_deco

def test_local_cacher():
    @local_cacher("test_cacher", cache_folder="./__test_cache__")
    def test_action_function(command:str):
        ### Do something
        result = command.split(" ")[1]
        return result
    
    print(test_action_function(command = "test_action_function test"))
    print(test_action_function(command = "test_action_function test"))

if __name__ == "__main__":
    test_local_cacher()

