<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { PlusCircleIcon, TrashIcon } from '@heroicons/vue/20/solid'
import Button from 'primevue/button';
import ConfirmDialog from 'primevue/confirmdialog';
import { useConfirm } from "primevue/useconfirm";

//
type ProjectLocations = string[]

interface AppConfig {
  locations: ProjectLocations
}

const appConfig: Ref<AppConfig | undefined> = ref()
const projectLocations: Ref<ProjectLocations> = ref([])
const confirm = useConfirm();

async function addProjectLocation() {
  projectLocations.value = await invoke<ProjectLocations>("add_project_location");
}

async function deleteProjectLocation(location: string) {
  projectLocations.value = await invoke<ProjectLocations>("delete_project_location", { location })
}

async function getAppConfig() {
  appConfig.value = await invoke<AppConfig>("get_app_config")
  projectLocations.value = appConfig.value.locations
}

async function confirmDeleteLocation(location: string) {
  confirm.require({
    header: 'Confirm delete location',
    message: 'Are you sure you want to delete this location?',
    accept: () => deleteProjectLocation(location),
    acceptLabel: "Yes, I'm sure",
    acceptProps: {},
    rejectLabel: "No, cancel",
    rejectProps: {
      style: 'outlined',
      severity: "danger" 
    }
  })
}

onBeforeMount(async () => {
  await getAppConfig()
})
</script>

<template>
  <div class="container h-screen">
    <div class="flex h-full">
      <div className="bg-zinc-800 w-[300px] p-2">

        <div className="flex items-center mb-3">
          <div className="flex-1 text-xl">
            Project Locations
          </div>
          <Button rounded class="p-[2px]" @click="addProjectLocation">
            <PlusCircleIcon class="size-6" />
          </Button>
        </div>

        <div>
          <ul class="divide-solid divide-slate-600 divide-y-[1px] border-y-[1px] border-slate-600">
            <li class="group cursor-pointer  py-3" v-for="(location, index) in projectLocations" :key="`proj_loc_${index}`">
              <div class="flex items-center">
                <div class="flex-1">
                  {{ location }}
                </div>
                <div class="opacity-20 group-hover:opacity-100" @click="confirmDeleteLocation(location)">
                  <TrashIcon class="size-6" />
                </div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <ConfirmDialog></ConfirmDialog>
  </div>
</template>

<style scoped></style>
