interface StorageConfig {
  type: 'localStorage' | 'sessionStorage'
  prefix: string
  expire: number
  isEncrypt?: boolean // 是否加密存储
}

interface StorageData {
  value: string
  time: number
  expire: number
}

const defaultConfig: StorageConfig = {
  type: 'localStorage',
  prefix: '12.09',
  expire: 0,
}

export const setStorage = (key: string, value: string, expire = 0): void => {
  expire = (expire ? expire : defaultConfig.expire) * 1000
  const data: StorageData = {
    value: value,
    time: Date.now(),
    expire: expire,
  }
  const encryptString: string = JSON.stringify(data)
  window[defaultConfig.type].setItem(autoAddPrefix(key), encryptString)
}

export const getStorage = (key: string): string | null => {
  key = autoAddPrefix(key)
  const storageString: string | null = window[defaultConfig.type].getItem(key)
  if (!storageString || storageString === 'null') {
    return null
  }
  const storage = JSON.parse(storageString) as StorageData
  const nowTime = Date.now()

  if (storage.expire && storage.expire < nowTime - storage.time) {
    removeStorage(key)
    return null
  } else {
    return storage.value
  }
}

export const hasStorage = (key: string): boolean => {
  key = autoAddPrefix(key)
  const arr = getStorageAll().filter(item => {
    return item.key === key
  })
  return arr.length > 0
}

export const getStorageKeys = (): string[] => {
  const items = getStorageAll()
  const keys: string[] = []
  for (let index = 0; index < items.length; index++) {
    keys.push(items[index].key)
  }
  return keys
}

export const getStorageForIndex = (index: number): string | null => {
  return window[defaultConfig.type].key(index) || null
}

export const getStorageLength = (): number => {
  return window[defaultConfig.type].length
}

export const getStorageAll = (): { key: string; val: string }[] => {
  const len = getStorageLength()
  const arr: { key: string; val: string }[] = []
  for (let i = 0; i < len; i++) {
    const key = window[defaultConfig.type].key(i)
    if (key !== null) {
      const getKey = autoRemovePrefix(key)
      const storageString = window[defaultConfig.type].getItem(key)
      if (storageString) {
        const storage = JSON.parse(storageString) as StorageData
        const nowTime = Date.now()
        if (storage.expire && nowTime - storage.time > storage.expire) {
          removeStorage(getKey)
        } else {
          const getVal: string = storage.value

          arr.push({ key: getKey, val: getVal })
        }
      }
    }
  }
  return arr
}

export const removeStorage = (key: string): void => {
  window[defaultConfig.type].removeItem(autoAddPrefix(key))
}

export const clearStorage = (): void => {
  window[defaultConfig.type].clear()
}

const autoAddPrefix = (key: string): string => {
  const prefix = defaultConfig.prefix ? defaultConfig.prefix + '_' : ''
  return prefix + key
}

const autoRemovePrefix = (key: string): string => {
  const len = defaultConfig.prefix ? defaultConfig.prefix.length + 1 : 0
  return key.substr(len)
}
