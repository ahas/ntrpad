export interface Project {
  items: ProjectTreeItem[];
}

export interface ProjectTreeItem {
  type: string;
  key: string;
  name: string;
  absolutePath: string;
  parent?: ProjectTreeItem;
  children?: ProjectTreeItem[];
}

export interface PhotoItem extends ProjectTreeItem {
  directoryName: string;
  htmlFilePath: string;
  fpvImageFilePath: string;
  location: PhotoLocation;
  address: PhotoAddress;
}

export interface PhotoLocation {
  latitude: number | null;
  longitude: number | null;
  direction: number | null;
}

export type PhotoAddress = Record<string, string>;

export interface PhotoItemGroup extends ProjectTreeItem {}

export const useProjectStore = defineStore("project", () => {
  const _items = ref<ProjectTreeItem[]>([]);

  function findItemByKey(key: string, items: ProjectTreeItem[] = _items.value): ProjectTreeItem | null {
    for (const item of items) {
      if (item.key === key) {
        return item;
      }

      if (item.children) {
        const found = findItemByKey(key, item.children);

        if (found) {
          return found;
        }
      }
    }

    return null;
  }

  return {
    items: _items,

    findItemByKey,
  };
});
