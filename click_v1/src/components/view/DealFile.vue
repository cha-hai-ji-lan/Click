<template>
     <div class="main-contain">
          <div class="title">
               <div class="tag-icon tooltip" data-tooltip="选择文件" @click="() => { openFileDialog() }">
                    <svg t="1766902940874" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="11272" width="200" height="200">
                         <path d="M682.624 693.312a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM682.624 522.688a32 32 0 1 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM426.624 352a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h85.312z"
                              p-id="11273"></path>
                         <path d="M603.648 53.952a32.128 32.128 0 0 1 10.56 4.288l1.536 0.96a31.808 31.808 0 0 1 4.224 3.456l256 256a32.064 32.064 0 0 1 9.408 22.656v512A117.44 117.44 0 0 1 768 970.624H256a117.312 117.312 0 0 1-117.312-117.312V170.624A117.376 117.376 0 0 1 256 53.312h341.376l6.272 0.64zM256 117.312a53.376 53.376 0 0 0-53.312 53.312v682.688a53.312 53.312 0 0 0 53.312 53.312h512a53.44 53.44 0 0 0 53.376-53.312v-480H597.312a32 32 0 0 1-32-32v-224H256z m373.312 192h146.816L629.312 162.496v146.816z"
                              p-id="11274"></path>
                    </svg>
               </div>
               <div class="tag-icon wire tooltip" data-tooltip="选择文件夹" @click="() => { openFolderDialog() }">
                    <svg t="1766725619065" class="icon" viewBox="0 0 1329 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="10656" width="200" height="200">
                         <path d="M1036.572 951.784h-780.721c-62.416 0-113.411-47.899-115.072-107.38l-78.125-495.818v-2.735c0-60.873 51.662-110.396 115.164-110.396h14.961v-63.036c0-60.873 51.651-110.396 115.13-110.396h234.199c42.067 0 105.428 34.714 140.178 74.338h302.237c48.717 0 161.228 35.661 166.943 104.891 45.468 14.762 78.311 56.069 78.311 104.599v2.747l-78.137 495.807c-1.659 59.493-52.643 107.38-115.072 107.38zM136.887 343.559l78.008 495.070v2.747c0 21.109 18.374 38.279 40.957 38.279h780.721c22.582 0 40.957-17.17 40.957-38.279v-2.747l78.008-495.070c-1.274-20.046-19.146-35.989-40.932-35.989h-37.075v-59.903c-1.823-3.846-12.027-14.026-34.48-24.113-23.096-10.357-46.52-15.078-58.536-15.078h-341.532l-10.87-17.147c-17.953-28.368-70.714-57.191-90-57.191h-234.199c-22.593 0-40.968 17.182-40.968 38.279v135.152h-89.135c-21.775 0-39.647 15.942-40.921 36z"
                              p-id="10657"></path>
                         <path d="M100.875 306.542h1086.428v69.779h-1086.428z" p-id="10658"></path>
                    </svg>
               </div>
               <div class="tag-icon wire tooltip" data-tooltip="路径池" @click="() => { click('choose-path-pool') }">
                    <svg t="1766718881525" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="8409" width="200" height="200">
                         <path d="M516.973714 950.857143a67.584 67.584 0 0 1-30.427428-7.606857l-111.616-53.248a64.073143 64.073143 0 0 1-38.034286-58.368V499.346286v-2.56L100.937143 179.712a65.609143 65.609143 0 0 1-5.046857-71.021714c10.166857-22.893714 32.914286-35.547429 58.368-35.547429h611.328c25.380571 0 48.274286 12.653714 58.368 35.547429 10.166857 22.820571 10.166857 48.201143-5.12 68.461714L582.948571 494.226286v388.169143c0 22.820571-12.653714 43.154286-30.427428 55.808a55.442286 55.442286 0 0 1-35.547429 12.653714zM154.185143 133.997714c-2.486857 0-5.046857 0-5.046857 2.56v5.12l235.958857 317.074286a70.217143 70.217143 0 0 1 12.653714 40.594286v332.288c0 2.56 0 5.12 2.56 5.12l114.102857 53.248h5.12c2.56 0 2.56-2.56 2.56-5.12V499.346286c0-15.213714 5.046857-27.940571 12.653715-40.594286l235.885714-317.147429c2.56-2.486857 2.56-5.046857 0-5.046857 0-2.56-2.486857-2.56-5.046857-2.56H154.331429z m740.790857 482.011429H717.385143a29.257143 29.257143 0 0 1-30.427429-30.427429 29.257143 29.257143 0 0 1 30.427429-30.427428h177.590857a29.257143 29.257143 0 0 1 30.427429 30.427428 29.257143 29.257143 0 0 1-30.427429 30.427429z m0 126.829714H717.385143a29.257143 29.257143 0 0 1-30.427429-30.427428 29.257143 29.257143 0 0 1 30.427429-30.427429h177.590857a29.257143 29.257143 0 0 1 30.427429 30.427429 29.257143 29.257143 0 0 1-30.427429 30.427428z m0 126.829714H717.385143a29.257143 29.257143 0 0 1-30.427429-30.427428 29.257143 29.257143 0 0 1 30.427429-30.427429h177.590857a29.257143 29.257143 0 0 1 30.427429 30.427429 29.257143 29.257143 0 0 1-30.427429 30.427428z"
                              p-id="8410"></path>
                         <path d="M483.254857 949.76q16.822857 8.411429 33.718857 8.411429 22.162286 0 39.936-14.189715 33.353143-23.917714 33.353143-61.586285V496.64l234.422857-315.099429q24.868571-33.206857 5.924572-75.849142Q812.909714 65.828571 765.586286 65.828571H154.331429q-47.177143 0-64.950858 39.497143-22.528 40.96 5.851429 78.701715l234.422857 315.172571v332.434286q0 46.08 42.276572 64.950857l111.396571 53.174857z m33.718857 1.097143a55.442286 55.442286 0 0 0 35.547429-12.653714 69.705143 69.705143 0 0 0 30.427428-55.808V494.226286l235.885715-317.074286c15.286857-20.260571 15.286857-45.641143 5.12-68.461714-10.166857-22.893714-32.987429-35.547429-58.368-35.547429H154.331429c-25.380571 0-48.274286 12.653714-58.368 35.547429a65.609143 65.609143 0 0 0 5.12 70.948571l235.885714 317.147429v334.848c0 25.380571 15.213714 48.201143 38.034286 58.368l111.616 53.248c10.166857 5.12 20.333714 7.606857 30.427428 7.606857z m248.685715-816.859429c2.486857 0 4.973714 0 4.973714 2.56 2.56 0 2.56 2.56 0 5.12l-1.462857 2.048-0.146286 0.146286L534.674286 458.752a76.653714 76.653714 0 0 0-12.653715 40.594286v385.536c0 2.56 0 5.12-2.56 5.12h-5.12l-114.102857-53.321143c-2.56 0-2.56-2.486857-2.56-5.12V499.419429a70.217143 70.217143 0 0 0-12.653714-40.594286L149.138286 141.604571v-5.046857c0-2.56 2.56-2.56 5.12-2.56h611.328z m-607.597715 7.314286h603.721143L528.676571 454.656l-0.219428 0.292571q-13.677714 22.893714-13.677714 44.397715v382.683428l-109.714286-51.2V499.346286q0-26.112-14.116572-44.982857l-232.886857-313.051429z m559.323429 482.011429h177.590857q16.310857 0 27.062857-10.678858 10.678857-10.752 10.678857-27.062857 0-37.741714-37.741714-37.741714H717.385143q-37.741714 0-37.741714 37.741714t37.741714 37.741715z m0-7.314286h177.590857a29.257143 29.257143 0 0 0 30.427429-30.427429 29.257143 29.257143 0 0 0-30.427429-30.427428H717.385143a29.257143 29.257143 0 0 0-30.427429 30.427428 29.257143 29.257143 0 0 0 30.427429 30.427429z m0 134.144h177.590857q37.741714 0 37.741714-37.741714t-37.741714-37.741715H717.385143q-16.310857 0-27.062857 10.678857-10.678857 10.678857-10.678857 27.062858 0 37.741714 37.741714 37.741714z m0-7.314286h177.590857a29.257143 29.257143 0 0 0 30.427429-30.427428 29.257143 29.257143 0 0 0-30.427429-30.427429H717.385143a29.257143 29.257143 0 0 0-30.427429 30.427429 29.257143 29.257143 0 0 0 30.427429 30.427428z m0 134.144h177.590857q37.741714 0 37.741714-37.741714 0-16.310857-10.678857-27.062857-10.752-10.678857-27.062857-10.678857H717.385143q-16.310857 0-27.062857 10.678857-10.678857 10.752-10.678857 27.062857 0 37.741714 37.741714 37.741714z m0-7.314286h177.590857a29.257143 29.257143 0 0 0 30.427429-30.427428 29.257143 29.257143 0 0 0-30.427429-30.427429H717.385143a29.257143 29.257143 0 0 0-30.427429 30.427429 29.257143 29.257143 0 0 0 30.427429 30.427428z"
                              p-id="8411"></path>
                    </svg>
               </div>
               <div class="tag-icon wire tooltip" data-tooltip="操作栏" @click="() => { click('choose-function') }">
                    <svg t="1766718761251" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="7191" width="200" height="200">
                         <path d="M802.864762 131.462095a97.52381 97.52381 0 0 1 97.523809 97.52381v547.84a97.52381 97.52381 0 0 1-97.523809 97.523809H188.025905a97.52381 97.52381 0 0 1-97.52381-97.523809V228.985905a97.52381 97.52381 0 0 1 97.52381-97.52381h614.887619z m0 73.142857H188.025905a24.380952 24.380952 0 0 0-24.380953 24.380953v547.84a24.380952 24.380952 0 0 0 24.380953 24.380952h614.887619a24.380952 24.380952 0 0 0 24.380952-24.380952V228.985905a24.380952 24.380952 0 0 0-24.380952-24.380953z"
                              p-id="7192"></path>
                         <path d="M472.795429 147.456v707.047619h-73.142858v-707.047619zM354.889143 323.486476v73.142857h-170.666667v-73.142857zM354.889143 466.358857v73.142857h-170.666667v-73.142857z"
                              p-id="7193"></path>
                    </svg>
               </div>


          </div>
          <div class="explorer-act-path">
               <transition-group name="path-item" tag="div">

                    <div class="index-path" v-for="item in active_path" :key="item[1]">
                         <div class="choose">
                              <input type="checkbox" :value="item" :checked="isSelected(item)"
                                   @change="handleCheckboxChange(item)" />

                         </div>
                         <div class="name">{{ item[1] }}</div>
                         <div class="path">{{ item[0] }}</div>
                    </div>
               </transition-group>
          </div>
          <div class="oper">
               <div class="h3-head">
                    <span class="icon-left-margin tooltip" data-tooltip="工作台" @click="() => {
                         click('work-bench')
                    }">
                         <svg t="1766485462276" class="icon " viewBox="0 0 1024 1024" version="1.1"
                              xmlns="http://www.w3.org/2000/svg" p-id="8143" width="200" height="200">
                              <path d="M896.1 115H128c-35.3 0-63.9 28.6-64 63.9V689c-0.1 35.3 28.5 64 63.8 64H896.1c35.3 0 63.9-28.6 63.9-63.9V179c0-35.3-28.6-64-63.9-64z m-25.6 546.1H153.7V202h716.8v459.1zM64.1 840.2c0 35.3 28.6 63.9 63.9 64h768.2c35.3 0 63.9-28.6 63.9-63.9v-56.5H64l0.1 56.4z m409.1-13.1h77.7c9.6 0 17.1 7.6 17.1 16.8-0.1 9.3-7.7 16.9-17 16.8h-77.8c-9.3 0.2-17-7.3-17.2-16.6v-0.2c0-9.2 7.6-16.8 17.2-16.8z"
                                   p-id="8144"></path>
                         </svg>
                    </span>
                    <h3>操作</h3>
                    <span class="icon-right-margin tooltip" data-tooltip="指南 施工ing🔨">
                         <svg t="1766480075133" class="icon" viewBox="0 0 1024 1024" version="1.1"
                              xmlns="http://www.w3.org/2000/svg" p-id="6827" width="200" height="200">
                              <path d="M512.004454 1023.999898c-140.696348 0-276.682338-58.777077-373.142279-161.380964C41.071046 758.58146-7.875719 622.493071 1.033002 479.441544 17.00726 222.62463 222.62463 17.00726 479.441544 1.033002c143.051527-8.908721 279.139916 40.038044 383.17739 137.829173C965.120422 235.322117 1023.999898 371.308107 1023.999898 512.004454c0 168.446501-82.943262 326.141097-221.796426 421.884246a32.050915 32.050915 0 0 1-44.543604-8.191928 32.050915 32.050915 0 0 1 8.191927-44.543603A448.170091 448.170091 0 0 0 960.000467 512.004454c0-124.926888-50.175553-240.945056-141.208343-326.550693C727.759334 99.950522 608.669194 57.045303 483.33271 64.930033 258.771508 78.856309 78.856309 258.771508 64.930033 483.33271c-7.782331 125.336485 35.020488 244.426625 120.626127 335.459414 78.232904 83.250459 181.860781 132.299623 294.499779 140.184352v-46.898782c0-17.715042 14.335872-31.948516 31.948515-31.948516s31.948516 14.335872 31.948516 31.948516v79.973688A31.856356 31.856356 0 0 1 512.004454 1023.999898z"
                                   p-id="6828"></path>
                              <path d="M512.004454 832.001606c-17.715042 0-31.948516-14.335872-31.948515-31.948515v-50.073155c0-81.919271 45.669994-156.670606 119.192539-194.865465 56.524297-29.388538 90.520794-84.376849 88.677611-143.461124-2.457578-83.352858-79.154496-153.086638-170.904079-155.441816-50.380352-1.433587-99.327116 17.203047-133.835609 50.892347-30.514928 29.695736-47.308379 68.50499-47.308379 109.055029 0 17.715042-14.335872 31.948516-31.948516 31.948516-17.715042 0-31.948516-14.335872-31.948515-31.948516 0-58.060283 23.654189-113.048594 66.559407-154.929821 46.796384-45.567594 112.638998-70.75777 180.119997-69.016985 125.131686 3.071973 229.783555 100.658304 233.265124 217.393265 2.559977 83.864854-44.646003 161.380964-123.186104 202.135801-52.223535 27.238158-84.786445 80.076087-84.786445 138.13637v50.073155A31.876836 31.876836 0 0 1 512.004454 832.001606z"
                                   p-id="6829"></path>
                         </svg>
                    </span>
               </div>
               <div v-if="focusMethod !== ''" class="oper-1">
                    <div class="item-title">{{ (availableMethods as any)[`${focusMethod}`] }}</div>
               </div>
               <div v-if="focusMethod === '0'" class="oper-1">
                    <transition name="replace-name-transition" mode="out-in">
                         <DFChangeName :active_path="active_path" v-model:inputRefReplaceOldName="inputRefReplaceOldName" v-model:inputRefReplaceNewName="inputRefReplaceNewName" :SubmitRepluceName="SubmitRepluceName"></DFChangeName>
                         <!-- <div v-if="active_path && active_path.length > 0" class="replace-name">
                              <div class="item-title tooltip" data-tooltip="------示例 施工ing🔨">
                                   <svg t="1766481043465" class="icon" viewBox="0 0 1024 1024" version="1.1"
                                        xmlns="http://www.w3.org/2000/svg" p-id="7054" width="200" height="200">
                                        <path d="M512 145.92c201.728 0 366.08 164.352 366.08 366.08s-164.352 366.08-366.08 366.08S145.92 713.728 145.92 512 310.272 145.92 512 145.92M512 87.04c-234.496 0-424.96 190.464-424.96 424.96s189.952 424.96 424.96 424.96c234.496 0 424.96-190.464 424.96-424.96S746.496 87.04 512 87.04L512 87.04z"
                                             p-id="7055"></path>
                                        <path d="M578.56 721.92c-16.384 0-29.184-13.312-29.184-29.184l0-40.96c0-12.288 7.68-23.04 18.944-27.648 64-23.552 107.008-84.992 107.008-153.088 0-90.112-73.216-163.328-163.328-163.328-90.112 0-163.328 73.216-163.328 163.328 0 68.096 43.008 129.536 107.008 153.088 11.776 4.096 18.944 15.36 18.944 27.648l0 40.96c0 16.384-13.312 29.184-29.184 29.184-16.384 0-29.184-13.312-29.184-29.184L416.256 670.72c-76.288-36.352-126.464-114.688-126.464-200.192 0-122.368 99.328-221.696 221.696-221.696s221.696 99.328 221.696 221.696c0 85.504-50.176 163.328-126.464 200.192l0 21.504C607.744 708.608 594.432 721.92 578.56 721.92z"
                                             p-id="7056"></path>
                                        <path d="M578.56 742.912" p-id="7057"></path>
                                        <path d="M445.952 742.912" p-id="7058"></path>
                                        <path d="M559.104 818.688 465.408 818.688c-16.384 0-29.184-13.312-29.184-29.184s13.312-29.184 29.184-29.184l93.696 0c16.384 0 29.184 13.312 29.184 29.184S574.976 818.688 559.104 818.688z"
                                             p-id="7059"></path>
                                   </svg>
                              </div>
                              <div class="item-title tooltip" data-tooltip="配置 施工ing🔨">
                                   <svg t="1766504502772" class="icon" viewBox="0 0 1024 1024" version="1.1"
                                        xmlns="http://www.w3.org/2000/svg" p-id="9296" width="200" height="200">
                                        <path d="M815.98 379.92c21.84-0.2 41.83-12.04 51.39-31.48l77.52-157.47c10.42-21.18 6.46-46.86-9.88-63.91L894.3 84.57c-18.3-19.09-46.97-22.94-69.62-9.4l-156.63 93.49c-16.94 10.12-27.47 28.67-27.47 48.41v70.13l-100.32 96.14c-30.02-32.81-56.01-61.27-69.8-76.49 10.82-91.02-6.52-157.53-51.59-197.79-63.69-56.86-158.93-40.02-196.93-30.12A37.72 37.72 0 0 0 194.92 106c-3.44 13.3 0.62 27.45 10.58 36.91l62.45 59.41c-1.18 16.28-8.15 31.38-19.89 42.97-10.64 10.5-24.11 17.02-38.66 18.86l-68.29-66.01c-10.3-9.94-25.07-13.21-38.63-8.54-13.49 4.66-23.08 16.34-25.05 30.45-6.29 45.12-13.59 156.76 54.53 218.73 40.81 37.14 99.25 48.39 173.87 33.53 2.06 2.16 4.34 4.57 6.79 7.14 13.14 13.81 29.93 31.45 43.35 44.9L114.99 765.29c-22.6 22.6-36.69 51.38-39.68 81.04-3.19 31.61 6.92 61.67 27.74 82.48 18.28 18.29 43.72 28.31 71.06 28.31 3.78 0 7.59-0.19 11.42-0.57 29.66-2.99 58.44-17.09 81.03-39.68l252.02-252.01c9.58 10.32 21.64 23.9 32.87 38.29-11.86 95.35 5.6 164.39 51.94 205.3 34.38 30.35 77.67 39.1 116.13 39.1 32.77 0 62.02-6.34 79.31-11.09 13.2-3.63 23.4-14.15 26.64-27.45 3.24-13.34-1.01-27.42-11.08-36.71l-60.43-55.97a61.251 61.251 0 0 1 18.28-42.72 61.258 61.258 0 0 1 38.8-17.45c17.16 17.36 45.84 46.04 61.23 59.6 9.95 8.77 23.99 11.7 36.59 7.56 12.69-4.12 22.38-14.75 25.28-27.72 21.94-97.55 10.13-167.95-35.08-209.25-58.66-53.6-151.71-37.85-196.52-25.9l-70.59-76.98 109.16-104.59 74.87 1.04zM358.8 435.47c-8.68-9.12-15.47-16.23-18.03-18.74-9.26-9.07-22.36-12.66-35.09-9.68-59.9 14.22-103.91 9.01-130.81-15.46-25.92-23.59-36.17-65.75-37.37-108.22l35.6 34.42a37.609 37.609 0 0 0 25.8 10.67c34.92 0.47 68.73-12.92 93.94-37.76 25.2-24.85 39.08-58.02 39.08-93.42v-5.95c0-10.33-4.3-20.34-11.75-27.39l-32.68-31.08c29.48-1.76 64.73 2.19 88.91 23.79 30.15 26.93 40.01 78.55 29.33 153.37-1.6 11.04 1.7 22.1 9.08 30.37 8.99 10.07 41.01 45.15 79.39 87.1l-12.5 11.98-20.44-20.44-60.21 60.21c-12.98-13.01-29.99-30.89-42.25-43.77zM221.47 871.78c-23.12 23.09-56.67 28.54-73.32 11.94-9.87-9.88-10.11-23.66-9.37-30.99 1.49-14.85 9.26-30.27 21.31-42.33l301.16-301.17 61.38 61.38-301.16 301.17z m445.07-256.21c9.97 10.83 25.43 14.91 39.29 10.37 10.73-3.44 106.26-32.64 150.22 7.51 23.92 21.85 27.62 61.79 23.68 101.23a3202.444 3202.444 0 0 1-30.7-30.87c-6.35-6.48-14.86-10.45-23.98-11.18-36.18-2.94-71.68 10.09-97.52 35.51a124.98 124.98 0 0 0-37.11 96.93l0.35 5.65c0.66 9.74 4.98 18.72 12.04 25.19l29.6 27.42c-28.91 1.85-63.07-1.73-86.8-22.68-38.51-34-37.34-105.29-29.57-159.16 1.4-9.82-1.17-20.05-7.11-28.12-15.2-20.54-32.45-39.95-45.22-53.62l49.13-49.13-35.15-35.15 8.2-7.86 80.65 87.96z m32.65-284.84l-166.6 159.62-5.77-5.77 160.11-153.44c11.09-10.59 17.44-25.46 17.44-40.78v-69.1l147.95-88.31 33.06 34.5-73.16 148.62-73.27-1.02c-15.21-0.32-29.05 5.38-39.76 15.68z"
                                             p-id="9297"></path>
                                   </svg>
                              </div>
                              <input class="input-box" type="text" v-model="inputRefReplaceOldName" placeholder="旧字段">
                              <input class="input-box" type="text" v-model="inputRefReplaceNewName" placeholder="新字段">
                              <div class="submit-replace-name" @click="() => { SubmitRepluceName('replace-name') }">修改
                              </div>
                         </div>
                         <div v-else class="place-holder">
                              <span>🔎 当前无可观察路径...</span>
                         </div> -->
                    </transition>
               </div>
               <div v-if="focusMethod === '1'" class="oper-1">
                    <transition name="replace-name-transition" mode="out-in">
                         <div v-if="active_path && active_path.length > 0" class="order-and-replace-name">
                              <div class="item-title tooltip" data-tooltip="------示例 施工ing🔨">
                                   <svg t="1766481043465" class="icon" viewBox="0 0 1024 1024" version="1.1"
                                        xmlns="http://www.w3.org/2000/svg" p-id="7054" width="200" height="200">
                                        <path d="M512 145.92c201.728 0 366.08 164.352 366.08 366.08s-164.352 366.08-366.08 366.08S145.92 713.728 145.92 512 310.272 145.92 512 145.92M512 87.04c-234.496 0-424.96 190.464-424.96 424.96s189.952 424.96 424.96 424.96c234.496 0 424.96-190.464 424.96-424.96S746.496 87.04 512 87.04L512 87.04z"
                                             p-id="7055"></path>
                                        <path d="M578.56 721.92c-16.384 0-29.184-13.312-29.184-29.184l0-40.96c0-12.288 7.68-23.04 18.944-27.648 64-23.552 107.008-84.992 107.008-153.088 0-90.112-73.216-163.328-163.328-163.328-90.112 0-163.328 73.216-163.328 163.328 0 68.096 43.008 129.536 107.008 153.088 11.776 4.096 18.944 15.36 18.944 27.648l0 40.96c0 16.384-13.312 29.184-29.184 29.184-16.384 0-29.184-13.312-29.184-29.184L416.256 670.72c-76.288-36.352-126.464-114.688-126.464-200.192 0-122.368 99.328-221.696 221.696-221.696s221.696 99.328 221.696 221.696c0 85.504-50.176 163.328-126.464 200.192l0 21.504C607.744 708.608 594.432 721.92 578.56 721.92z"
                                             p-id="7056"></path>
                                        <path d="M578.56 742.912" p-id="7057"></path>
                                        <path d="M445.952 742.912" p-id="7058"></path>
                                        <path d="M559.104 818.688 465.408 818.688c-16.384 0-29.184-13.312-29.184-29.184s13.312-29.184 29.184-29.184l93.696 0c16.384 0 29.184 13.312 29.184 29.184S574.976 818.688 559.104 818.688z"
                                             p-id="7059"></path>
                                   </svg>
                              </div>
                              <div class="item-title tooltip" data-tooltip="配置 施工ing🔨">
                                   <svg t="1766504502772" class="icon" viewBox="0 0 1024 1024" version="1.1"
                                        xmlns="http://www.w3.org/2000/svg" p-id="9296" width="200" height="200">
                                        <path d="M815.98 379.92c21.84-0.2 41.83-12.04 51.39-31.48l77.52-157.47c10.42-21.18 6.46-46.86-9.88-63.91L894.3 84.57c-18.3-19.09-46.97-22.94-69.62-9.4l-156.63 93.49c-16.94 10.12-27.47 28.67-27.47 48.41v70.13l-100.32 96.14c-30.02-32.81-56.01-61.27-69.8-76.49 10.82-91.02-6.52-157.53-51.59-197.79-63.69-56.86-158.93-40.02-196.93-30.12A37.72 37.72 0 0 0 194.92 106c-3.44 13.3 0.62 27.45 10.58 36.91l62.45 59.41c-1.18 16.28-8.15 31.38-19.89 42.97-10.64 10.5-24.11 17.02-38.66 18.86l-68.29-66.01c-10.3-9.94-25.07-13.21-38.63-8.54-13.49 4.66-23.08 16.34-25.05 30.45-6.29 45.12-13.59 156.76 54.53 218.73 40.81 37.14 99.25 48.39 173.87 33.53 2.06 2.16 4.34 4.57 6.79 7.14 13.14 13.81 29.93 31.45 43.35 44.9L114.99 765.29c-22.6 22.6-36.69 51.38-39.68 81.04-3.19 31.61 6.92 61.67 27.74 82.48 18.28 18.29 43.72 28.31 71.06 28.31 3.78 0 7.59-0.19 11.42-0.57 29.66-2.99 58.44-17.09 81.03-39.68l252.02-252.01c9.58 10.32 21.64 23.9 32.87 38.29-11.86 95.35 5.6 164.39 51.94 205.3 34.38 30.35 77.67 39.1 116.13 39.1 32.77 0 62.02-6.34 79.31-11.09 13.2-3.63 23.4-14.15 26.64-27.45 3.24-13.34-1.01-27.42-11.08-36.71l-60.43-55.97a61.251 61.251 0 0 1 18.28-42.72 61.258 61.258 0 0 1 38.8-17.45c17.16 17.36 45.84 46.04 61.23 59.6 9.95 8.77 23.99 11.7 36.59 7.56 12.69-4.12 22.38-14.75 25.28-27.72 21.94-97.55 10.13-167.95-35.08-209.25-58.66-53.6-151.71-37.85-196.52-25.9l-70.59-76.98 109.16-104.59 74.87 1.04zM358.8 435.47c-8.68-9.12-15.47-16.23-18.03-18.74-9.26-9.07-22.36-12.66-35.09-9.68-59.9 14.22-103.91 9.01-130.81-15.46-25.92-23.59-36.17-65.75-37.37-108.22l35.6 34.42a37.609 37.609 0 0 0 25.8 10.67c34.92 0.47 68.73-12.92 93.94-37.76 25.2-24.85 39.08-58.02 39.08-93.42v-5.95c0-10.33-4.3-20.34-11.75-27.39l-32.68-31.08c29.48-1.76 64.73 2.19 88.91 23.79 30.15 26.93 40.01 78.55 29.33 153.37-1.6 11.04 1.7 22.1 9.08 30.37 8.99 10.07 41.01 45.15 79.39 87.1l-12.5 11.98-20.44-20.44-60.21 60.21c-12.98-13.01-29.99-30.89-42.25-43.77zM221.47 871.78c-23.12 23.09-56.67 28.54-73.32 11.94-9.87-9.88-10.11-23.66-9.37-30.99 1.49-14.85 9.26-30.27 21.31-42.33l301.16-301.17 61.38 61.38-301.16 301.17z m445.07-256.21c9.97 10.83 25.43 14.91 39.29 10.37 10.73-3.44 106.26-32.64 150.22 7.51 23.92 21.85 27.62 61.79 23.68 101.23a3202.444 3202.444 0 0 1-30.7-30.87c-6.35-6.48-14.86-10.45-23.98-11.18-36.18-2.94-71.68 10.09-97.52 35.51a124.98 124.98 0 0 0-37.11 96.93l0.35 5.65c0.66 9.74 4.98 18.72 12.04 25.19l29.6 27.42c-28.91 1.85-63.07-1.73-86.8-22.68-38.51-34-37.34-105.29-29.57-159.16 1.4-9.82-1.17-20.05-7.11-28.12-15.2-20.54-32.45-39.95-45.22-53.62l49.13-49.13-35.15-35.15 8.2-7.86 80.65 87.96z m32.65-284.84l-166.6 159.62-5.77-5.77 160.11-153.44c11.09-10.59 17.44-25.46 17.44-40.78v-69.1l147.95-88.31 33.06 34.5-73.16 148.62-73.27-1.02c-15.21-0.32-29.05 5.38-39.76 15.68z"
                                             p-id="9297"></path>
                                   </svg>
                              </div>
                              <SelectorBar class="select-bar" v-model="selectedValue" :options="selectOptions" placeholder="排序方式">
                              </SelectorBar>
                              <input class="input-box" type="text" v-model="inputRefSortName"
                                   placeholder="<起始标志>固定名称{原名}...">
                              <!-- <input class="input-box" type="text" v-model="inputRefReplaceOldName" placeholder="旧字段">
                              <input class="input-box" type="text" v-model="inputRefReplaceNewName" placeholder="新字段"> -->
                              <div class="submit-replace-name"
                                   @click="() => { SubmitRepluceName('order-replace-name') }">修改
                              </div>
                         </div>
                         <div v-else class="place-holder">
                              <span>🔎 当前无可观察路径...</span>
                         </div>
                    </transition>
               </div>

          </div>
     </div>



     <DFWorkbenchFW v-model:FloatingWindow="FloatingWindow" :click="click"> </DFWorkbenchFW>

     <DFPathPoolFW :userSelectedShortPath="userSelectedShortPath" :selectFolder="selectFolder"
          v-model:FloatingWindow="FloatingWindow" :click="click" :openFileDialog="openFileDialog"
          :openFolderDialog="openFolderDialog"></DFPathPoolFW>
     <DFChooseMethoadFW v-model:FloatingWindow="FloatingWindow" :click="click" :changeMethod="changeMethod">
     </DFChooseMethoadFW>

</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
import { open } from '@tauri-apps/plugin-dialog';
import { type PathItem } from "../../class/PathIndex"
import { type FloatingWindowState } from '../../class/PathIndex';
import { ref, reactive, onMounted, onUnmounted } from "vue";
import SelectorBar from '../components/SelectorBar.vue'
import { parseStringToArray } from '../../util/DataTool'
import DFChooseMethoadFW from "./components-view/DFChooseMethoadFW.vue";
import DFPathPoolFW from "./components-view/DFPathPoolFW.vue";
import DFWorkbenchFW from "./components-view/DFWorkbenchFW.vue";
import DFChangeName from "./components-view/DFChangeName.vue"
// import { B } from "vue-router/dist/router-CWoNjPRp.mjs";


const active_path = ref<PathItem[] | null>(null)
const selectedPaths = ref<PathItem[]>([]) // 选中的路径

let pollingInterval = ref<number | null>(null);
// 保存上一次获取的数据用于比较
const lastActivePathData = ref<string | null>(null);

const inputRefReplaceOldName = ref("")
const inputRefReplaceNewName = ref("")
const inputRefSortName = ref("")

const FloatingWindow = reactive<FloatingWindowState>({
     "choose-path-pool": false,  // 设置双标志关闭窗口为了让关闭动画正常显示
     "choose-path-pool-close": false,
     "choose-function": false,
     "choose-function-close": false,
     "work-bench": false,
     "work-bench-close": false,
})
const userSelectedPath = ref<string[] | null>(null)
const userSelectedShortPath = ref<string[] | null>(null)
const floatingWindowElement = ref<HTMLElement | null>(null);

const selectFolder = ref(false)
const selectedValue = ref('')
const focusMethod = ref<string>("")
const availableMethods = reactive({
     "0": "修改名字字段",
     "1": "改名排序"
})
const selectOptions = [
     { value: 'by-name', label: '按原名称顺序排序' },
     { value: 'by-time', label: '按修改时间排序' },
     { value: 'by-size', label: '按大小排序' },
     { value: 'picture-sort', label: '图片排序' },
]



defineProps({
     mainColor: {
          type: Object,
          default: () => ({})
     }
});

const get_explorer_active_path = async () => {
     try {
          const data = await invoke<PathItem[] | null>("active_explorer_path");
          // 将新数据转换为 JSON 字符串以便比较
          const newDataString = JSON.stringify(data);

          // 只有当数据真正改变时才更新 active_path
          if (lastActivePathData.value !== newDataString) {
               lastActivePathData.value = newDataString;
               active_path.value = data;
          }
     } catch (err) {
          console.error("无法处理文件:", err);
     }
};

// 检查项目是否被选中
const isSelected = (item: PathItem) => {
     return selectedPaths.value.some(selectedItem =>
          selectedItem[0] === item[0] && selectedItem[1] === item[1]
     );
};

// 处理复选框变化
const handleCheckboxChange = (item: PathItem) => {
     const index = selectedPaths.value.findIndex(selectedItem =>
          selectedItem[0] === item[0] && selectedItem[1] === item[1]
     );

     if (index === -1) {
          // 如果未选中，则添加到选中列表
          selectedPaths.value.push(item);
     } else {
          // 如果已选中，则从选中列表移除
          selectedPaths.value.splice(index, 1);
     }
};
const startPolling = () => {
     // 每隔1秒调用一次
     pollingInterval.value = setInterval(get_explorer_active_path, 1000);
};

const stopPolling = () => {
     if (pollingInterval.value) {
          clearInterval(pollingInterval.value);
          pollingInterval.value = null;
     }
};

onMounted(() => {
     get_explorer_active_path();
     startPolling();
     click('choose-function'); // 默认打开操作窗口
});

onUnmounted(() => {
     stopPolling();
});


const openFolderDialog = async () => {
     selectFolder.value = true;
     let default_path = 'C:\\'  // 设置默认打开路径，可以根据需要修改
     if (selectedPaths.value && selectedPaths.value.length > 0) {
          default_path = selectedPaths.value[0][0]
     }
     try {
          userSelectedPath.value = await open({
               title: "选取文件夹",
               multiple: true,
               directory: true,
               filters: [{
                    name: 'All Files',
                    extensions: ['*']
               }],
               defaultPath: default_path
          });
          userSelectedShortPath.value = []

          if (userSelectedPath.value === null) {
               console.log('用户取消了选择');
          } else {
               if (userSelectedShortPath.value?.length === 0) {
                    userSelectedPath.value.forEach((single_path) => {
                         userSelectedShortPath.value?.push(getShortPath(single_path))
                    })
                    console.log('选中的文件:', userSelectedPath.value);
                    // 在这里可以处理选中的文件，比如将其添加到 active_path 中
               }

          }
     } catch (error) {
          console.error('打开文件对话框时出错:', error);
     }
};
const openFileDialog = async () => {
     selectFolder.value = false;
     let default_path = 'C:\\'  // 设置默认打开路径，可以根据需要修改
     if (selectedPaths.value && selectedPaths.value.length > 0) {
          default_path = selectedPaths.value[0][0]
     }
     try {
          userSelectedPath.value = await open({
               title: "选取文件",
               multiple: true,
               // directory: true,
               filters: [{
                    name: 'All Files',
                    extensions: ['*']
               }],
               defaultPath: default_path
          });
          userSelectedShortPath.value = []

          if (userSelectedPath.value === null) {
               console.log('用户取消了选择');
          } else {
               if (userSelectedShortPath.value?.length === 0) {
                    userSelectedPath.value.forEach((single_path) => {
                         userSelectedShortPath.value?.push(getShortPath(single_path))
                    })
                    console.log('选中的文件:', userSelectedPath.value);
                    // 在这里可以处理选中的文件，比如将其添加到 active_path 中
               }

          }
     } catch (error) {
          console.error('打开文件对话框时出错:', error);
     }
};

const changeMethod = (methodName: string) => {
     focusMethod.value = methodName
}
const click = (whichOne: string, index: number = 0) => {
     switch (whichOne) {
          case 'choose-path-pool':
               if (FloatingWindow["choose-path-pool"] === false) {
                    FloatingWindow["choose-path-pool"] = true
                    if (floatingWindowElement.value && parseFloat(floatingWindowElement.value.style.left) < 0) {
                         floatingWindowElement.value.style.left = `0px`;
                    }
               } else {
                    FloatingWindow["choose-path-pool-close"] = true;
                    setTimeout(() => {
                         FloatingWindow["choose-path-pool"] = false;
                         FloatingWindow["choose-path-pool-close"] = false;
                    }, 500)
               }
               break;

          case 'choose-function':
               if (FloatingWindow["choose-function"] === false) {
                    FloatingWindow["choose-function"] = true
               } else {
                    FloatingWindow["choose-function-close"] = true;
                    setTimeout(() => {
                         FloatingWindow["choose-function"] = false;
                         FloatingWindow["choose-function-close"] = false;
                    }, 500)
               }
               break;
          case 'close-path-item':
               if (userSelectedPath.value) {
                    userSelectedPath.value.splice(index, 1);
                    if (userSelectedShortPath.value) {
                         userSelectedShortPath.value.splice(index, 1);
                    }
               }
               break;
          case 'close-path-item-all':
               userSelectedPath.value = []
               userSelectedShortPath.value = []
               break;
          case 'work-bench':
               if (FloatingWindow["work-bench"] === false) {
                    FloatingWindow["work-bench"] = true
               } else {
                    FloatingWindow["work-bench-close"] = true;
                    setTimeout(() => {
                         FloatingWindow["work-bench"] = false;
                         FloatingWindow["work-bench-close"] = false;
                    }, 500)
               }
               break;
          default:
               break;
     }
}

const SubmitRepluceName = (tag: String) => {
     switch (tag) {
          case 'replace-name':
               if (inputRefReplaceOldName.value !== "") {
                    if (selectedPaths.value.length !== 0) {
                         selectedPaths.value.forEach((item, _) => {
                              invoke("replace_all_name", { dirPath: item[0], oldNameSign: inputRefReplaceOldName.value, newNameSign: inputRefReplaceNewName.value })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.log(selectedPaths.value)
                                        console.error("无法处理文件:", err);
                                   });
                         });
                    } else if (userSelectedPath.value?.length !== 0) {  // 添加对跳跃文件夹下的文件进行重命名
                         userSelectedPath.value?.forEach((item, _) => { // TODO:: 写应对路径池的标志改名
                              invoke("replace_all_name", { dirPath: item[0], oldNameSign: inputRefReplaceOldName.value, newNameSign: inputRefReplaceNewName.value })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.error("无法处理文件:", err);
                                   });
                         });

                    }


               } else {
                    alert("请输入要替换的字段")
               }

               break;

          case 'order-replace-name':
               const participle = parseStringToArray(inputRefSortName.value)
               if (selectedPaths.value.length !== 0) {
                    console.log(selectedValue.value)
                    switch (selectedValue.value) {
                         case 'by-name':
                              selectedPaths.value.forEach((item, _) => {
                                   invoke("change_file_name", { rule: participle, path: item[1], mode: 1, oldToNew: true, orderMode: 2 })
                                        .then(() => {
                                             console.log("成功替换文件名");
                                        })
                                        .catch((err) => {
                                             console.log(selectedPaths.value)
                                             console.error("无法处理文件:", err);
                                        });
                              });
                              break;
                         case 'by-time':
                              selectedPaths.value.forEach((item, _) => {
                                   invoke("change_file_name", { rule: participle, path: item[1], mode: 1, oldToNew: true, orderMode: 1 })
                                        .then(() => {
                                             console.log("成功替换文件名");
                                        })
                                        .catch((err) => {
                                             console.log(selectedPaths.value)
                                             console.error("无法处理文件:", err);
                                        });
                                   userSelectedPath.value = []  // 经行了名称修改会消耗掉用户选择的文件，所以需要重新获取
                                   userSelectedShortPath.value = []
                              });
                              break;
                         case 'by-size':
                              console.log("施工。。。")
                              break;
                         case 'picture-sort':
                              console.log("施工。。。")
                              break;

                         default:
                              selectedPaths.value.forEach((item, _) => {
                                   invoke("change_file_name", { rule: participle, path: item[0], mode: 1, oldToNew: true, orderMode: 1 })
                                        .then(() => {
                                             console.log("成功替换文件名");
                                        })
                                        .catch((err) => {
                                             console.log(selectedPaths.value)
                                             console.error("无法处理文件:", err);
                                        });
                              });
                              break;
                    }
               } else if (userSelectedPath.value?.length !== 0) {  // 添加路径池中文件进行重命名
                    switch (selectedValue.value) {
                         case 'by-name':
                              console.log(userSelectedPath.value)
                              invoke("change_pool_file_name", { rule: participle, path: userSelectedPath.value, mode: 1, oldToNew: true, orderMode: 2 })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.error("无法处理文件:", err);
                                   });
                              userSelectedPath.value = []  // 经行了名称修改会消耗掉用户选择的文件，所以需要重新获取
                              userSelectedShortPath.value = []
                              break;
                         case 'by-time':
                              console.log(userSelectedPath.value)
                              invoke("change_pool_file_name", { rule: participle, path: userSelectedPath.value, mode: 1, oldToNew: true, orderMode: 1 })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.error("无法处理文件:", err);
                                   });
                              userSelectedPath.value = []  // 经行了名称修改会消耗掉用户选择的文件，所以需要重新获取
                              userSelectedShortPath.value = []
                              break;
                         case 'by-size':
                              console.log("施工。。。")
                              break;
                         case 'picture-sort':
                              console.log("施工。。。")
                              break;

                         default:
                              console.log(userSelectedPath.value)
                              invoke("change_pool_file_name", { rule: participle, path: userSelectedPath.value, mode: 1, oldToNew: true, orderMode: 1 })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.error("无法处理文件:", err);
                                   });
                              userSelectedPath.value = []  // 经行了名称修改会消耗掉用户选择的文件，所以需要重新获取
                              userSelectedShortPath.value = []
                              break;
                    }
               }

               break

          default:
               break;
     }

}

// 新增函数：提取路径的最后两个目录部分
const getShortPath = (fullPath: string): string => {
     // 移除末尾的反斜杠（如果有）
     const cleanPath = fullPath.replace(/[\/\\]$/, '');
     // 按照分隔符分割路径
     const parts = cleanPath.split(/[/\\]/);
     // 获取最后两个部分
     if (parts.length >= 2) {
          return parts.slice(-2).join('\\'); // 使用反斜杠连接
     } else {
          return cleanPath;
     }
}



</script>
<style scoped>
h3 {
     width: 100%;
     margin: 1vh 0;
     font-size: 3.5vmin;
     text-align: center;
     user-select: none;
     /* 用户无法选择 */
     -webkit-user-select: none;
     /* Safari兼容性 */
     -moz-user-select: none;
     /* Firefox兼容性 */
     -ms-user-select: none;
     /* IE兼容性 */
}

.main-contain {
     width: 100%;
     display: flex;
     justify-content: start;
     align-items: center;
     flex-direction: column;
     transition: all;
     margin-right: 1vw;
     margin-bottom: 1vw;

}

.title {
     display: flex;
     height: 4%;
     margin-top: 2%;
     width: 100%;
     justify-content: start;
     align-items: center;
     flex-direction: row;
     position: relative;
}

.title::after {
     content: '';
     position: absolute;
     bottom: -2px;
     left: 0;
     width: 100%;
     height: 2px;
     background: linear-gradient(to right, var(--normal-attention-color) 20%, var(--active-attention-color) 40%, var(--main-border) 60%, var(--main-back-ground) 80%);
}


.tag-icon {
     width: 4vmin;
     height: 4vmin;
     display: flex;
     position: relative;
     justify-content: start;
     align-items: center;
}


.icon-right-margin {
     margin-right: 2.5vmin;
}

.icon-left-margin {
     margin-left: 2.5vmin;
}


.small-icon:active {
     animation: active-icon 0.25s forwards;
     animation-timing-function: linear;
}

.wire::before {
     content: "";
     position: absolute;
     /* top: 50%; */
     left: -0.35vmin;
     width: 0.2vmin;
     border-radius: 0.1vmin;
     max-width: 2px;
     height: 2vmin;
     max-height: 20px;
     background-color: var(--main-border);
     /* transform: translateY(50%); */
}

.explorer-act-path {
     margin-top: 2%;
     width: 100%;
     height: 22%;
     overflow-y: auto;
     /* scrollbar-width: thin; "auto" 或 "thin" */
     /* scrollbar-color: var(--unite-but-color) var(--title-bar-lg-2); 滑块颜色 轨道颜色 */


}

/* Webkit内核浏览器的滚动条样式（Chrome, Safari, Edge） */
.explorer-act-path::-webkit-scrollbar {
     width: 1vmin;
     /* 垂直滚动条宽度 */
     height: 1vmin;
     /* 水平滚动条高度 */

}

.explorer-act-path::-webkit-scrollbar-track {
     background: var(--title-bar-lg-1);
     /* 滚动条轨道背景色 */
     border-radius: 4px;
}

.explorer-act-path::-webkit-scrollbar-thumb {
     background: var(--normal-attention-color);
     /* 滚动条滑块颜色 */
     border-radius: 4px;
}

.explorer-act-path::-webkit-scrollbar-thumb:hover {
     background: var(--active-attention-color);
     /* 滑块悬停时的颜色 */
}

.index-path {
     display: grid;
     grid-template-columns: 5% 20% 75%;
     position: relative;
     font-size: 2vmin;
}

.index-path:nth-child(1)::before {
     content: '';
     position: absolute;
     top: 0;
     left: 0;
     width: 100%;
     height: 2px;
     height: 2px;
     background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow));
}

.index-path::after {
     content: '';
     position: absolute;
     bottom: 0;
     left: 0;
     width: 100%;
     height: 2px;
     background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow));
}

.choose {
     display: flex;
     justify-content: center;
     align-items: center;
}

/* 自定义复选框样式 */
.choose input[type="checkbox"] {
     appearance: none;
     width: 2vmin;
     height: 2vmin;
     border: 2px solid #ccc;
     border-radius: 3px;
     position: relative;
     cursor: pointer;
     display: flex;
     justify-content: center;
     align-items: center;
     transition: all 0.2s ease;
}

/* 添加悬停效果 */
.choose input[type="checkbox"]:hover {
     border-color: var(--normal-attention-color);
     box-shadow: 0 0 4px rgba(30, 144, 255, 0.3);
}

/* 添加焦点效果 */
.choose input[type="checkbox"]:focus {
     outline: none;
     box-shadow: 0 0 6px rgba(30, 144, 255, 0.5);
}

.choose input[type="checkbox"]:checked {
     background-color: var(--normal-attention-color);
     border-color: var(--normal-attention-color);
}

.choose input[type="checkbox"]:checked::after {
     content: '';
     position: absolute;
     width: 0.6vmin;
     height: 0.4vmin;
     border: solid white;
     border-width: 0 0 0.2vmin 0.2vmin;
     transform: rotate(-45deg);
}

.name {
     height: 4vh;
     max-height: 30px;
     min-height: 10px;
     display: flex;
     justify-content: center;
     align-items: center;
     flex-direction: row;
     overflow: hidden;
     text-overflow: ellipsis;
     white-space: nowrap;
     border-right: 2px solid var(--title-min-icon-hover-shadow);
     border-left: 2px solid var(--title-min-icon-shadow);
     -webkit-box-orient: vertical;
}

.path {
     height: 4vh;
     max-height: 30px;
     min-height: 10px;
     display: flex;
     justify-content: center;
     align-items: center;
     flex-direction: row;
     overflow: hidden;
     text-overflow: ellipsis;
     white-space: nowrap;
     -webkit-box-orient: vertical;

}

.path::before {
     content: "";
     position: absolute;
     /* top: 50%; */
     left: -0.5vw;
     width: 0.2vw;
     max-width: 2px;
     height: 3vh;
     max-height: 20px;
     background-color: var(--main-border);
     /* transform: translateY(50%); */
}

.oper {
     display: flex;
     width: 100%;
     /* flex:1; */
     box-sizing: border-box;
     border-radius: 1vmin;
     justify-content: start;
     align-items: center;
     flex-direction: column;
     border: 1px solid var(--unite-but-color);
     padding: 2vmin;
     background: var(--title-bar-lg-2);
     /* 画格子模拟纸张 */
     background-image:
          linear-gradient(to right, var(--button-color) 1px, transparent 1px),
          linear-gradient(to bottom, var(--button-color) 1px, transparent 1px);
     background-size: var(--grid-size) var(--grid-size);
     overflow-y: auto;
     overflow-x: hidden;
     animation: show-paper 0.5s ease-in-out forwards;

}

.h3-head {
     display: flex;
     justify-content: center;
     align-items: center;
     width: 100%;
     position: relative;


}

.h3-head::after {
     content: '';
     position: absolute;
     bottom: 4px;
     left: 12.5%;
     width: 75%;
     height: 2px;
     background: radial-gradient(circle at center, var(--font-color), var(--button-color), var(--title-bar-lg-2));
}

.oper-1 {
     margin: 0.75vh 0;
     text-align: center;
     display: flex;
     height: 4vmin;
     width: 100%;
     font-size: 2.5vmin;
     /* border-radius: 1vmin; */
     justify-content: center;
     align-items: center;
     flex-direction: column;
     border-top: 1px dashed var(--unite-but-color);
     border-bottom: 1px dashed var(--unite-but-color);
     animation: show-method 0.5s ease-in-out forwards;
}

.replace-name {
     font-size: 2vmin;
     display: grid;
     width: 100%;
     height: 4vmin;
     grid-template-columns: 5% 5% 37.5% 37.5% 15%;

}

.order-and-replace-name {
     font-size: 2vmin;
     display: grid;
     display: grid;
     width: 100%;
     height: 4vmin;
     grid-template-columns: 5% 5% 37.5% 37.5% 15%;
}

.place-holder {
     width: 100%;
     height: 4vmin;
     font-size: medium;
     display: flex;
     justify-content: center;
     align-items: center;
     color: var(--font-color);

}

.item-title {
     height: 4vmin;
     display: flex;
     justify-content: center;
     align-items: center;
     user-select: none;
     /* 用户无法选择 */
     -webkit-user-select: none;
     /* Safari兼容性 */
     -moz-user-select: none;
     /* Firefox兼容性 */
     -ms-user-select: none;
     /* IE兼容性 */
     border-bottom: 1px dashed var(--unite-but-color);
}
.select-bar {
     height: 4vmin;

}
.input-box {
     font-family: "楷体", 'Courier New', Courier, monospace;
     font-size: 2vmin;
     text-align: center;
     height: 4vmin;
     padding: 0px 0px;
     margin: auto 0;
     background: transparent;
     border-top: 0px dashed var(--unite-but-color);
     border-bottom: 2px dashed var(--unite-but-color);
     border-left: 0px dashed var(--unite-but-color);
     border-right: 0px dashed var(--unite-but-color);
     transition: all 0.3s ease;
}
.input-box:hover {
     border-color: var(--positive-show-color);
}

.input-box:focus {
     outline: none;
     /* 取消默认的焦点轮廓 */
     /* border: 1px solid var(--active-attention-color); */
     border-bottom: 2px dashed var(--active-attention-color);
     box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);

}

.submit-replace-name {
     border-radius: 1vmin;
     height: 4vmin;
     border-left: 2px solid var(--active-attention-color);
     border-right: 2px solid var(--active-attention-color);
     display: flex;
     justify-content: center;
     align-items: center;
     user-select: none;
     /* 用户无法选择 */
     -webkit-user-select: none;
     /* Safari兼容性 */
     -moz-user-select: none;
     /* Firefox兼容性 */
     -ms-user-select: none;
     /* IE兼容性 */
     transition: 0.25s;
}

.submit-replace-name:hover {
     box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
     filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));

}

.submit-replace-name:active {
     background: var(--title-min-icon-shadow);
     box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
     animation: active-icon 0.25s forwards;
     animation-timing-function: linear;
}

/* ----------------------------------------动画区----------------------------------------------- */

/* 替换名称区域的过渡动画 */
.replace-name-transition-leave-active {
     transition: all 0.25s ease;
}

.replace-name-transition-leave-from {
     opacity: 1;
     filter: blur(0px);
}

.replace-name-transition-leave-to {
     opacity: 0;
     filter: blur(10px);
}

/* 列表项的过渡动画 */
.path-item-enter-active,
.path-item-leave-active {
     transition: all 0.25s ease;
}

.path-item-leave-active {
     transition-delay: 0.25s;
     /* 进入动画延迟 */
}

.path-item-enter-from {
     opacity: 0;
     filter: blur(10px);
}

.path-item-enter-to {
     opacity: 1;
     filter: blur(0px);
}

.path-item-leave-from {
     opacity: 1;
     filter: blur(0px);
}

.path-item-leave-to {
     opacity: 0;
     filter: blur(10px);
}

/* 确保离开的元素在动画期间保持定位
.path-item-move {
     transition: transform 0.5s ease;
} */


@keyframes show-method {
     0% {
          opacity: 0;
          filter: blur(10px);
     }

     100% {
          opacity: 1;
          filter: blur(0px);
     }
}


@keyframes show-paper {
     0% {
          height: 0%;
     }

     100% {
          height: 78%;
     }

}
</style>