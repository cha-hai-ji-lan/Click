<!-- 用于 DealFile 转换选定路径下的格式转换 -->
<template>
    <div class="format-conversion-container">
        <div v-if="active_path && active_path.length > 0" class="order-and-replace-name">
            <div class="item-title tooltip" data-tooltip="------示例 施工ing🔨">
                <IconInfo></IconInfo>
            </div>
            <div class="item-title tooltip" data-tooltip="配置" @click="() => { open_configure() }">
                <svg t="1766504502772" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="9296" width="200" height="200">
                    <path
                        d="M815.98 379.92c21.84-0.2 41.83-12.04 51.39-31.48l77.52-157.47c10.42-21.18 6.46-46.86-9.88-63.91L894.3 84.57c-18.3-19.09-46.97-22.94-69.62-9.4l-156.63 93.49c-16.94 10.12-27.47 28.67-27.47 48.41v70.13l-100.32 96.14c-30.02-32.81-56.01-61.27-69.8-76.49 10.82-91.02-6.52-157.53-51.59-197.79-63.69-56.86-158.93-40.02-196.93-30.12A37.72 37.72 0 0 0 194.92 106c-3.44 13.3 0.62 27.45 10.58 36.91l62.45 59.41c-1.18 16.28-8.15 31.38-19.89 42.97-10.64 10.5-24.11 17.02-38.66 18.86l-68.29-66.01c-10.3-9.94-25.07-13.21-38.63-8.54-13.49 4.66-23.08 16.34-25.05 30.45-6.29 45.12-13.59 156.76 54.53 218.73 40.81 37.14 99.25 48.39 173.87 33.53 2.06 2.16 4.34 4.57 6.79 7.14 13.14 13.81 29.93 31.45 43.35 44.9L114.99 765.29c-22.6 22.6-36.69 51.38-39.68 81.04-3.19 31.61 6.92 61.67 27.74 82.48 18.28 18.29 43.72 28.31 71.06 28.31 3.78 0 7.59-0.19 11.42-0.57 29.66-2.99 58.44-17.09 81.03-39.68l252.02-252.01c9.58 10.32 21.64 23.9 32.87 38.29-11.86 95.35 5.6 164.39 51.94 205.3 34.38 30.35 77.67 39.1 116.13 39.1 32.77 0 62.02-6.34 79.31-11.09 13.2-3.63 23.4-14.15 26.64-27.45 3.24-13.34-1.01-27.42-11.08-36.71l-60.43-55.97a61.251 61.251 0 0 1 18.28-42.72 61.258 61.258 0 0 1 38.8-17.45c17.16 17.36 45.84 46.04 61.23 59.6 9.95 8.77 23.99 11.7 36.59 7.56 12.69-4.12 22.38-14.75 25.28-27.72 21.94-97.55 10.13-167.95-35.08-209.25-58.66-53.6-151.71-37.85-196.52-25.9l-70.59-76.98 109.16-104.59 74.87 1.04zM358.8 435.47c-8.68-9.12-15.47-16.23-18.03-18.74-9.26-9.07-22.36-12.66-35.09-9.68-59.9 14.22-103.91 9.01-130.81-15.46-25.92-23.59-36.17-65.75-37.37-108.22l35.6 34.42a37.609 37.609 0 0 0 25.8 10.67c34.92 0.47 68.73-12.92 93.94-37.76 25.2-24.85 39.08-58.02 39.08-93.42v-5.95c0-10.33-4.3-20.34-11.75-27.39l-32.68-31.08c29.48-1.76 64.73 2.19 88.91 23.79 30.15 26.93 40.01 78.55 29.33 153.37-1.6 11.04 1.7 22.1 9.08 30.37 8.99 10.07 41.01 45.15 79.39 87.1l-12.5 11.98-20.44-20.44-60.21 60.21c-12.98-13.01-29.99-30.89-42.25-43.77zM221.47 871.78c-23.12 23.09-56.67 28.54-73.32 11.94-9.87-9.88-10.11-23.66-9.37-30.99 1.49-14.85 9.26-30.27 21.31-42.33l301.16-301.17 61.38 61.38-301.16 301.17z m445.07-256.21c9.97 10.83 25.43 14.91 39.29 10.37 10.73-3.44 106.26-32.64 150.22 7.51 23.92 21.85 27.62 61.79 23.68 101.23a3202.444 3202.444 0 0 1-30.7-30.87c-6.35-6.48-14.86-10.45-23.98-11.18-36.18-2.94-71.68 10.09-97.52 35.51a124.98 124.98 0 0 0-37.11 96.93l0.35 5.65c0.66 9.74 4.98 18.72 12.04 25.19l29.6 27.42c-28.91 1.85-63.07-1.73-86.8-22.68-38.51-34-37.34-105.29-29.57-159.16 1.4-9.82-1.17-20.05-7.11-28.12-15.2-20.54-32.45-39.95-45.22-53.62l49.13-49.13-35.15-35.15 8.2-7.86 80.65 87.96z m32.65-284.84l-166.6 159.62-5.77-5.77 160.11-153.44c11.09-10.59 17.44-25.46 17.44-40.78v-69.1l147.95-88.31 33.06 34.5-73.16 148.62-73.27-1.02c-15.21-0.32-29.05 5.38-39.76 15.68z"
                        p-id="9297"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="图片格式" @click="() => { chooseMode('img') }">
                <svg t="1769356546322" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5340" width="200" height="200">
                    <path
                        d="M938.666667 553.92V768c0 64.8-52.533333 117.333333-117.333334 117.333333H202.666667c-64.8 0-117.333333-52.533333-117.333334-117.333333V256c0-64.8 52.533333-117.333333 117.333334-117.333333h618.666666c64.8 0 117.333333 52.533333 117.333334 117.333333v297.92z m-64-74.624V256a53.333333 53.333333 0 0 0-53.333334-53.333333H202.666667a53.333333 53.333333 0 0 0-53.333334 53.333333v344.48A290.090667 290.090667 0 0 1 192 597.333333a286.88 286.88 0 0 1 183.296 65.845334C427.029333 528.384 556.906667 437.333333 704 437.333333c65.706667 0 126.997333 16.778667 170.666667 41.962667z m0 82.24c-5.333333-8.32-21.130667-21.653333-43.648-32.917333C796.768 511.488 753.045333 501.333333 704 501.333333c-121.770667 0-229.130667 76.266667-270.432 188.693334-2.730667 7.445333-7.402667 20.32-13.994667 38.581333-7.68 21.301333-34.453333 28.106667-51.370666 13.056-16.437333-14.634667-28.554667-25.066667-36.138667-31.146667A222.890667 222.890667 0 0 0 192 661.333333c-14.464 0-28.725333 1.365333-42.666667 4.053334V768a53.333333 53.333333 0 0 0 53.333334 53.333333h618.666666a53.333333 53.333333 0 0 0 53.333334-53.333333V561.525333zM320 480a96 96 0 1 1 0-192 96 96 0 0 1 0 192z m0-64a32 32 0 1 0 0-64 32 32 0 0 0 0 64z"
                        p-id="5341"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="视频格式" @click="() => { chooseMode('vid') }">
                <svg t="1770809708808" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5343" xmlns:xlink="http://www.w3.org/1999/xlink"
                    width="200" height="200">
                    <path
                        d="M387.398 325.052l308.427 178.843L387.398 684.32V325.05z m-1.707-72.642c-6.981-4.05-14.879-4.965-22.17-3.32-2.235-0.512-4.531-0.838-6.905-0.838-17.005 0-30.797 13.777-30.797 30.782v450.42c0 17.005 13.792 30.798 30.797 30.798 2.607 0 5.09-0.42 7.51-1.04a30.13 30.13 0 0 0 23.086-3.118l371.293-217.212c11.109-6.501 14.52-12.544 15.003-31.108 0.483-18.564 0.217-30.379-13.203-38.152L385.691 252.41zM527.39 960.764h-31.03C118.628 958.212 61.5 836.652 61.5 502.291c0-324.754 71.37-439.29 434.86-439.29h31.03c363.489 0 434.858 114.536 434.858 439.29 0 334.361-57.126 455.921-434.858 458.473M496.36 0C91.88 0-0.001 140.854-0.001 502.228c0 361.389 70.718 518.985 498.207 521.71h27.338c427.488-2.725 498.207-160.321 498.207-521.71C1023.751 140.854 931.87 0 527.39 0h-31.03z"
                        fill="#333333" p-id="5344"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="音频格式" @click="() => { chooseMode('aud') }">
                <svg t="1771170847136" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="8788" width="200" height="200">
                    <path
                        d="M640 970.666667H384c-118.186667 0-198.272-25.002667-251.946667-78.72S53.333333 758.186667 53.333333 640V384c0-118.186667 25.002667-198.272 78.72-251.946667S265.813333 53.333333 384 53.333333h256c118.186667 0 198.272 25.002667 251.946667 78.72S970.666667 265.813333 970.666667 384v256c0 118.186667-25.002667 198.272-78.72 251.946667S758.186667 970.666667 640 970.666667z m-256-853.333334c-100.096 0-165.802667 19.2-206.72 59.946667S117.333333 283.904 117.333333 384v256c0 100.096 19.072 165.802667 59.946667 206.72S283.904 906.666667 384 906.666667h256c100.096 0 165.802667-19.072 206.72-59.946667S906.666667 740.096 906.666667 640V384c0-100.096-19.072-165.802667-59.946667-206.72S740.096 117.333333 640 117.333333z"
                        p-id="8789"></path>
                    <path
                        d="M410.453333 525.226667a122.453333 122.453333 0 1 1-122.453333 122.453333 122.581333 122.581333 0 0 1 122.453333-122.453333z m0 180.906666a58.453333 58.453333 0 1 0-58.453333-58.453333 58.538667 58.538667 0 0 0 58.453333 58.453333z"
                        p-id="8790"></path>
                    <path
                        d="M500.906667 679.68a32 32 0 0 1-32-32V331.52a32 32 0 0 1 32-32 32 32 0 0 1 32 32v316.16a32 32 0 0 1-32 32z"
                        p-id="8791"></path>
                    <path
                        d="M542.293333 253.738667a87.936 87.936 0 0 1 28.16 4.778666l99.712 33.237334A100.010667 100.010667 0 0 1 736 383.146667v26.325333a73.216 73.216 0 0 1-73.386667 77.952 87.338667 87.338667 0 0 1-28.16-4.821333l-99.712-33.28A100.010667 100.010667 0 0 1 468.906667 357.973333v-26.453333a73.216 73.216 0 0 1 73.386666-77.781333z m120.32 169.685333c2.304 0 9.386667 0 9.386667-13.824v-26.453333a37.205333 37.205333 0 0 0-22.058667-30.634667l-99.84-33.322667a24.576 24.576 0 0 0-7.808-1.450666c-2.346667 0-9.386667 0-9.386666 13.781333v26.453333a37.205333 37.205333 0 0 0 22.058666 30.634667l99.84 33.28h0.256a23.722667 23.722667 0 0 0 7.552 1.536z"
                        p-id="8792"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="办公格式" @click="() => { chooseMode('office') }">
                <svg t="1771418868907" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5334" width="200" height="200">
                    <path
                        d="M288 320h448a32 32 0 0 0 0-64H288a32 32 0 0 0 0 64zM288 544h448a32 32 0 0 0 0-64H288a32 32 0 0 0 0 64zM544 704H288a32 32 0 0 0 0 64h256a32 32 0 0 0 0-64z"
                        p-id="5335"></path>
                    <path
                        d="M896 132.928C896 77.28 851.552 32 796.928 32H227.04C172.448 32 128 77.28 128 132.928v758.144C128 946.72 172.448 992 227.04 992h435.008c1.568 0 2.912-0.672 4.416-0.896 8.96 1.6 18.464-0.256 25.984-6.528l192-160a31.424 31.424 0 0 0 10.816-27.2c0.16-1.184 0.736-2.208 0.736-3.424V132.928zM192 891.072V132.928C192 112.576 207.712 96 227.04 96h569.888C816.288 96 832 112.576 832 132.928V736h-96a96 96 0 0 0-96 96v96H227.04C207.712 928 192 911.424 192 891.072zM814.016 800L704 891.68V832a32 32 0 0 1 32-32h78.016z"
                        p-id="5336"></path>
                </svg>
            </div>
            <div class="占位符"></div>
            <div class="submit-replace-name" @click="() => { subCov() }">转化
            </div>
        </div>
        <div v-else class="place-holder">
            <span>🔎 当前无可观察路径...</span>
        </div>
        <transition name="replace-name-transition" mode='out-in'>
            <div v-if="formatMode === 'img'" class="conversion-item">
                <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="img_sourceOption"
                    placeholder="原始格式">
                </SelectorBar>
                <div class="text">转成</div>
                <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="img_aimOption"
                    placeholder="目标格式">
                </SelectorBar>
            </div>
            <div v-else-if="formatMode === 'vid'" class="conversion-item">
                <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="vid_sourceOption"
                    placeholder="原始格式">
                </SelectorBar>
                <div class="text">转成</div>
                <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="vid_aimOption"
                    placeholder="目标格式">
                </SelectorBar>
            </div>
            <div v-else-if="formatMode === 'aud'" class="conversion-item">
                <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="aud_sourceOption"
                    placeholder="原始格式">
                </SelectorBar>
                <div class="text">转成</div>
                <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="aud_aimOption"
                    placeholder="目标格式">
                </SelectorBar>
            </div>
            <div v-else-if="formatMode === 'office'" class="conversion-item">
                <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="office_sourceOption"
                    placeholder="原始格式">
                </SelectorBar>
                <div class="text">转成</div>
                <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="office_aimOption"
                    placeholder="目标格式">
                </SelectorBar>
            </div>
        </transition>
        <transition name="replace-name-transition" mode='in-out'>
            <div v-if="open_configure_interface" class="configer">
                <div v-if="cov_type === 'img-format-conversion'" class="items-list">
                    <ul>
                        <li class="choose-item bubble" data-bubtip="直接书写magick指令参数" :class="{}"
                            @click="addImgComponent('__bash__')">命令行</li>
                        <li class="choose-item bubble" data-bubtip="设置图片极限尺寸" :class="{}"
                            @click="addImgComponent('-resize')">调整尺寸
                        </li>
                        <li class="choose-item bubble" data-bubtip="修剪图片(需要锚点)"
                            :class="{ 'anchor-point': anchor_point }" @click="addImgComponent('-crop')">
                            裁剪大小</li>
                        <li class="choose-item bubble" data-bubtip="如果有指令需要锚点,应先设置锚点" :class="{}"
                            @click="addImgComponent('-gravity')">基准锚点</li>
                        <li class="choose-item bubble" data-bubtip="缩放图片尺寸" :class="{}"
                            @click="addImgComponent('-scale')">
                            缩放图片</li>
                        <!-- <li class="choose-item bubble" data-bubtip="扩展画布的指令可能需要先设置我" :class="{}"
                            @click="addImgComponent('-background')">设背景色</li> -->
                        <li class="choose-item bubble" data-bubtip="可能会扩展画布,如有需要应先设置背景颜色" :class="{}"
                            @click="addImgComponent('-rotate')">旋转图片</li>
                        <li class="choose-item bubble" data-bubtip="左右翻转或上下翻转" :class="{}"
                            @click="addImgComponent('-flip')">翻转图片</li>


                        <li class="claen-oper" @click="clearComponents()">清空操作</li>
                    </ul>
                </div>
                <div v-else-if="cov_type === 'vid-format-conversion'" class="items-list">
                    <span>施工中</span>
                    <ul>
                        <li class="choose-item bubble" data-bubtip="直接书写magick指令参数" :class="{}"
                            @click="addVidComponent('__bash__')">命令行</li>
                        <!-- <li class="choose-item bubble" data-bubtip="调整分辨率（缩放）" :class="{}" @click="addVidComponent('-vf')">
                            调分辨率
                        </li>
                        <li class="choose-item bubble" data-bubtip="调整输出帧率" :class="{}" @click="addVidComponent('-r')">
                            改变帧率</li> -->


                        <li class="claen-oper" @click="clearComponents()">清空操作</li>
                    </ul>
                </div>
                <div v-else-if="cov_type === 'aud-format-conversion'" class="items-list">
                    <span>施工中</span>
                    <ul>
                        <li class="choose-item bubble" data-bubtip="直接书写ffmpeg指令参数" :class="{}"
                            @click="addAudComponent('__bash__')">命令行</li>
                        <!-- <li class="choose-item bubble" data-bubtip="调整分辨率（缩放）" :class="{}" @click="addAudComponent('-vf')">
                            调分辨率
                        </li>
                        <li class="choose-item bubble" data-bubtip="调整输出帧率" :class="{}" @click="addAudComponent('-r')">
                            改变帧率</li> -->
                        <li class="claen-oper" @click="clearComponents()">清空操作</li>
                    </ul>
                </div>
                <div v-else-if="cov_type === 'office-format-conversion'" class="items-list">
                    <span>施工中</span>
                    <ul>
                        <li class="choose-item bubble" data-bubtip="直接书写magick指令参数" :class="{}"
                            @click="addOfficeComponent('__bash__')">命令行</li>
                        <!-- <li class="choose-item bubble" data-bubtip="调整分辨率（缩放）" :class="{}" @click="addAudComponent('-vf')">
                            调分辨率
                        </li>
                        <li class="choose-item bubble" data-bubtip="调整输出帧率" :class="{}" @click="addAudComponent('-r')">
                            改变帧率</li> -->
                        <li class="claen-oper" @click="clearComponents()">清空操作</li>
                    </ul>
                </div>
                <div class="items-group">
                    <div class="title">
                        <div class="tooltip" data-tooltip="示例 施工ing🔨">
                            <IconInfo></IconInfo>
                        </div>
                        <div class="text">
                            配置参数
                        </div>
                    </div>
                    <div class="w-box" v-for="(comp, index) in componentList" :key="comp.id">
                        <div class="w-box" v-if="cov_type === 'img-format-conversion'">
                            <div v-if="comp.type === '__bash__'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <input class="single-input" v-model="args_g2_nest[index as number][1]" type="text"
                                        placeholder='直接输入magick命令 如: -gravity southeast -pointsize 36 -fill white -annotate +20+20 "水印"'>
                                </transition>
                            </div>
                            <div v-else-if="comp.type === '-resize'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <div class="poer" v-if="args_mode[index as number] === 'nor-resize'">
                                        <input v-model="args_g2_nest[index as number][1]" type="text"
                                            placeholder="宽 例如:800 强制转化:800!">
                                        <input v-model="args_g2_nest[index as number][2]" type="text"
                                            placeholder="高 例如:400 强制转化:400!">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-resize', index as number) }">
                                            % 表示</div>
                                    </div>
                                    <div class="poer" v-else-if="args_mode[index as number] === 'percent-resize'">
                                        <input v-model="args_g2_nest[index as number][1]" type="text"
                                            placeholder="例如:80%">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-resize', index as number) }">
                                            像素表示</div>
                                    </div>
                                </transition>
                            </div>
                            <div v-else-if="comp.type === '-scale'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <div class="poer" v-if="args_mode[index as number] === 'nor-scale'">
                                        <input v-model="args_g2_nest[index as number][1]" type="text"
                                            placeholder="宽 例如:800 强制转化:800!">
                                        <input v-model="args_g2_nest[index as number][2]" type="text"
                                            placeholder="高 例如:400 强制转化:400!">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-scale', index as number) }">
                                            % 表示</div>
                                    </div>
                                    <div class="poer" v-else-if="args_mode[index as number] === 'percent-scale'">
                                        <input v-model="args_g2_nest[index as number][1]" type="text"
                                            placeholder="例如:80%">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-sacle', index as number) }">
                                            像素表示</div>
                                    </div>
                                </transition>
                            </div>
                            <div v-else-if="comp.type === '-gravity'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <transition name="replace-name-transition" mode='out-in'>
                                        <div class="choose-anchor-point">
                                            <div class="gravity-item">
                                                <span>顶部:</span>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'NorthWest' }"
                                                    @click="updateGravity(index as number, 'NorthWest')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'North' }"
                                                    @click="updateGravity(index as number, 'North')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'NorthEast' }"
                                                    @click="updateGravity(index as number, 'NorthEast')"></div>
                                            </div>
                                            <div class="gravity-item">
                                                <span>中部:</span>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'West' }"
                                                    @click="updateGravity(index as number, 'West')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'Center' }"
                                                    @click="updateGravity(index as number, 'Center')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'East' }"
                                                    @click="updateGravity(index as number, 'East')"></div>
                                            </div>
                                            <div class="gravity-item">
                                                <span>底部:</span>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'SouthWest' }"
                                                    @click="updateGravity(index as number, 'SouthWest')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'South' }"
                                                    @click="updateGravity(index as number, 'South')"></div>
                                                <div type="radio" :name="`radio-item${index}`" class="div-g-ele"
                                                    :class="{ 'choose-radio': args_g2_nest[index as number][1] === 'SouthEast' }"
                                                    @click="updateGravity(index as number, 'SouthEast')"></div>
                                            </div>
                                        </div>
                                    </transition>

                                    <!-- <input class="single-input" v-model="args_g2_nest[index as number][1]" type="text"
                                    placeholder='直接输入magick命令 如: -gravity southeast -pointsize 36 -fill white -annotate +20+20 "水印"'> -->
                                </transition>
                            </div>
                            <div v-else-if="comp.type === '-crop'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                    </div>
                                    <div class="tooltip" data-tooltip="关闭条目">
                                        <CloseIcon @click="() => { closeItem(index as number) }"></CloseIcon>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <template v-if="args_mode[index as number] === 'lt-crop' || 'ce-crop'">
                                        <div class="w-box">
                                            <div class="poer">
                                                <input v-model="args_g2_nest[index as number][1]" type="text"
                                                    placeholder="宽 如:800">
                                                <input v-model="args_g2_nest[index as number][2]" type="text"
                                                    placeholder="高 如:400">
                                                <!-- <div v-if="args_mode[index as number] === 'lt-crop'" class="inline-button"
                                                @click="() => { changStyle('crop', index as number) }">中心原点</div>
                                            <div v-if="args_mode[index as number] === 'ce-crop'" class="inline-button"
                                                @click="() => { changStyle('crop', index as number) }">左上原点</div> -->
                                            </div>
                                            <div class="poer">
                                                <input v-model="args_g2_nest[index as number][3]" type="text"
                                                    placeholder="原点偏移x 如:10">
                                                <input v-model="args_g2_nest[index as number][4]" type="text"
                                                    placeholder="原点偏移y 如:20">
                                            </div>
                                        </div>
                                    </template>
                                </transition>
                            </div>
                            <div v-if="comp.type === '-rotate'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <input class="single-input" v-model="args_g2_nest[index as number][1]" type="text"
                                        placeholder='输入旋转角度 如1顺时针旋转45°: 45  如2逆时针旋转45°: -45"'>
                                </transition>
                            </div>
                            <div v-else-if="comp.type === '-flip'" draggable="true" class="item">
                                <div class="name">
                                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                                        <IconInfo></IconInfo>
                                    </div>
                                    <div class="text">
                                        {{ args_item_title[index as number] ?? '__NAN_TITLE__' }}
                                        <div class="tooltip" data-tooltip="关闭条目">
                                            <CloseIcon @click="() => { closeItem(index as number) }">
                                            </CloseIcon>
                                        </div>
                                    </div>
                                </div>
                                <transition name="replace-name-transition" mode='out-in'>
                                    <div class="poer" v-if="args_g2_nest[index as number][0] === '-flip'">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-flip', index as number) }">
                                            左右翻转</div>
                                    </div>
                                    <div class="poer" v-else-if="args_g2_nest[index as number][0] === '-flop'">
                                        <div class="inline-button"
                                            @click="() => { changStyle('-flip', index as number) }">
                                            上下翻转</div>
                                    </div>
                                </transition>
                            </div>
                        </div>
                        <div class="w-box" v-else-if="cov_type === 'vid-format-conversion'">
                        </div>
                        <div class="w-box" v-else-if="cov_type === 'aud-format-conversion'">
                        </div>
                        <div class="w-box" v-else-if="cov_type === 'office-format-conversion'">
                        </div>

                    </div>
                </div>
            </div>
        </transition>
    </div>
</template>
<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import SelectorBar from '../../components/SelectorBar.vue'
import IconInfo from '../../../icon/icons/IconInfo.vue'
import CloseIcon from '../../../icon/icons/CloseIcon.vue'
import { imgExpendArgs1, imgExpendArgs2, vidExpendArgs1, vidExpendArgs2, audExpendArgs1, audExpendArgs2, cleanArgs, getAt, magick_args } from '../../../util/FormatCov'
import { img_sourceOption, img_aimOption, vid_sourceOption, vid_aimOption, aud_aimOption, aud_sourceOption, office_sourceOption, office_aimOption} from '../../../util/PluginObjects'
import { type PathItem } from '@/class/PathIndex'
const props = defineProps<{
    active_path: PathItem[] | null,
    formatSelectedValue: string[],
    inputRefSortName: string,
    SubmitRepluceName: (toolMethoad: string) => void
}
>()

const open_configure_interface = ref(false)

const cov_type = ref("format-conversion")
const selectedValueSon = ref(props.formatSelectedValue)
const formatMode = ref('')  // img 代表图片格式转换  vid 代表视频格式转换
const componentList = ref<any>([])  // 配置组件列表
// let componentIndex = 0  // 配置组件索引数
// const args_g1 :string[] = ["", ""]
/**
 * 
 * args_g2 参数索引对应表
 * img-----------------------------------------------------<br>
 * 0: 照片宽度  ||1: 照片高度   ||2:保留字用于调整||3: 照片裁剪宽 ||  
 * 4: 照片裁剪高||5: 左上角偏移x||6: 左上角偏移y||
 * ===========================================================
 */
const args_g2_nest = ref<any[]>([])
let args_g1: string[] = []
let args_g2: string[] = []
const args_mode = ref<string[]>([])
const args_item_title = ref<string[]>([])
const anchor_point = ref<boolean>(false)


const emit = defineEmits<{
    (e: 'update:formatSelectedValue', value: string[]): void,
}>()
watch(selectedValueSon, (newValue) => {
    console.log(newValue)
})
const combinedState = computed(() => ({
    selectedValue: selectedValueSon.value,
}));

// 监听组合状态的变化
watch(
    combinedState,
    (newState, oldState) => {
        // 检查具体是哪个值发生了变化
        if (newState.selectedValue !== oldState.selectedValue) {
            emit('update:formatSelectedValue', newState.selectedValue);

        }
    }
);

const subCov = () => {
    cleanArgs() // 执行前清除 过去可能存储的指令, 预防内存泄漏
    args2Unpack(args_g2_nest.value);
    if (cov_type.value === "img-format-conversion") {
        imgExpendArgs1(args_g1)
        imgExpendArgs2(args_g2);
    } else if (cov_type.value === "vid-format-conversion") {
        args_g1.push("-i")
        vidExpendArgs1(args_g1);
        vidExpendArgs2(args_g2);

    } else if (cov_type.value === "aud-format-conversion") {
        args_g1.push("-i")
        audExpendArgs1(args_g1);
        audExpendArgs2(args_g2);

    } else if (cov_type.value === "office-format-conversion") {
        audExpendArgs1(args_g1);
        audExpendArgs2(args_g2);

    }
    args_g1 = []  // 用一次清理一次防止内存泄露
    args_g2 = []
    props.SubmitRepluceName(cov_type.value);
}
const chooseMode = (input_formate: string) => {
    clearComponents() // 切换模式直接清空配置表
    switch (input_formate) {
        case 'img':
            cov_type.value = "img-format-conversion";
            break;
        case "vid":
            cov_type.value = "vid-format-conversion";
            break;
        case "aud":
            cov_type.value = "aud-format-conversion";
            break;
        case "office":
            cov_type.value = "office-format-conversion";
            break;
        default:
            break;
    }
    formatMode.value = input_formate;
}

const open_configure = () => {
    if (open_configure_interface.value === false) {
        open_configure_interface.value = true;
    } else {
        open_configure_interface.value = false;
    }
    console.log(open_configure_interface.value);

}

const changStyle = (mode: string, index: number) => {
    switch (mode) {
        case "-scale":
            if (args_mode.value[index] === "nor-scale") {
                args_mode.value[index] = "percent-scale"
                args_item_title.value[index] = "(横纵比🔒)缩放图像(%)"
                args_g2_nest.value[index] = ["-scale", "", ""]
            } else if (args_mode.value[index] === "percent-scale") {
                args_mode.value[index] = "nor-scale"
                args_item_title.value[index] = "(横纵比🔒)缩放图像(像素)"
                args_g2_nest.value[index] = ["-scale", "", ""]
            }
            break;
        case "-resize":
            if (args_mode.value[index] === "nor-resize") {
                args_mode.value[index] = "percent-resize"
                args_item_title.value[index] = "(横纵比🔒)调整图像尺寸(%)"
                args_g2_nest.value[index] = ["-resize", "", ""]
            } else if (args_mode.value[index] === "percent-resize") {
                args_mode.value[index] = "nor-resize"
                args_item_title.value[index] = "(横纵比🔒)调整图像尺寸(像素)"
                args_g2_nest.value[index] = ["-resize", "", ""]
            }
            break;
        case "-flip":
            if (args_g2_nest.value[index][0] === "-flip") {
                args_g2_nest.value[index][0] = "-flop"
                args_item_title.value[index] = "翻转图片(左右翻转)"
            } else if (args_g2_nest.value[index][0] === "-flop") {
                args_g2_nest.value[index][0] = "-flip"
                args_item_title.value[index] = "翻转图片(上下翻转)"

            }
            break;
        default:
            break;
    }

}
const args2Unpack = (args: any[]) => {
    console.log(args)
    args.forEach(group => {
        switch (group[0]) {
            case "__bash__":
                if (group[1] !== "") {
                    args_g2.push("__bash__")
                    const result = group[1].split(' ');
                    args_g2.push(...result)
                    args_g2.push("__end_bash__")
                }
                break;
            default:
                const subList = group.slice(1);
                console.log(group, subList)
                if (!subList.every((str: string) => str === "")) {
                    args_g2.push(...group)
                }
                break;
        }
    })

}
// 添加组件的方法
const addImgComponent = (type: string) => {
    if (componentList.value.length > 0) {
        if (getAt(componentList.value, -1).type === "-gravity" && magick_args["before-need-gravity"].includes(type)) {
            anchor_point.value = false;
        } else if (getAt(componentList.value, -1).type === "-gravity") {
            anchor_point.value = false;
            args_g2_nest.value.pop()
            args_item_title.value.pop()
            args_mode.value.pop()
            componentList.value.pop()
        }
        // if (getAt(componentList.value, -1).type === "-background" && magick_args["before-need-background"].includes(type)) {
        //     anchor_point.value = false;
        // } else if (getAt(componentList.value, -1).type === "-background") {
        //     anchor_point.value = false;
        //     args_g2_nest.value.pop()
        //     args_item_title.value.pop()
        //     args_mode.value.pop()
        //     componentList.value.pop()
        // }
    }
    switch (type) {
        case "__bash__":
            args_item_title.value.push("命令行")
            args_mode.value.push("__bash__")
            args_g2_nest.value.push(["__bash__", ""])
            break;
        case "-resize":
            args_item_title.value.push("(横纵比🔒)调整图像尺寸(像素)")
            args_mode.value.push("nor-resize")
            args_g2_nest.value.push(["-resize", "", ""])
            break;
        case "-scale":
            args_item_title.value.push("(横纵比🔒)缩放图像(像素)")
            args_mode.value.push("nor-scale")
            args_g2_nest.value.push(["-scale", "", ""])
            break;
        case "-gravity":
            anchor_point.value = true
            args_item_title.value.push("设置基准锚点(左上角)")
            args_mode.value.push("lt-gravity")
            args_g2_nest.value.push(["-gravity", ""])
            break;
        case "-rotate":
            args_item_title.value.push("旋转图片(顺时针|-逆时针)")
            args_mode.value.push("-rotate")
            args_g2_nest.value.push(["-rotate", ""])
            break;
        case "-crop":
            if (componentList.value.length > 0 && getAt(componentList.value, -1).type === "-gravity") {
                args_item_title.value.push("裁剪图像")
            }
            else {
                args_item_title.value.push("裁剪图像(无锚点:默认左上角)")
            }
            args_mode.value.push("lt-crop")
            args_g2_nest.value.push(["-crop", "", "", "", ""])
            break;
        case "-flip":
            args_item_title.value.push("翻转图片(上下翻转)")
            args_mode.value.push("-flip")
            args_g2_nest.value.push(["-flip"])
            break;
        default:
            break;
    }

    // 创建组件对象并添加到列表
    const newComponent = {
        type: type,
        id: Date.now() // 使用时间戳作为唯一 ID
    };
    componentList.value.push(newComponent);
}
const addVidComponent = (type: string) => {
    // if (componentList.value.length > 0) {
    //     if (getAt(componentList.value, -1).type === "-gravity" && magick_args["before-need-gravity"].includes(type)) {
    //         anchor_point.value = false;
    //     } else if (getAt(componentList.value, -1).type === "-gravity") {
    //         anchor_point.value = false;
    //         args_g2_nest.value.pop()
    //         args_item_title.value.pop()
    //         args_mode.value.pop()
    //         componentList.value.pop()
    //     }
    // if (getAt(componentList.value, -1).type === "-background" && magick_args["before-need-background"].includes(type)) {
    //     anchor_point.value = false;
    // } else if (getAt(componentList.value, -1).type === "-background") {
    //     anchor_point.value = false;
    //     args_g2_nest.value.pop()
    //     args_item_title.value.pop()
    //     args_mode.value.pop()
    //     componentList.value.pop()
    // }
    // }
    switch (type) {
        case "__bash__":
            args_item_title.value.push("命令行")
            args_mode.value.push("__bash__")
            args_g2_nest.value.push(["__bash__", ""])
            break;
        case "-resize":
            args_item_title.value.push("(横纵比🔒)调整图像尺寸(像素)")
            args_mode.value.push("nor-resize")
            args_g2_nest.value.push(["-resize", "", ""])
            break;
        case "-scale":
            args_item_title.value.push("(横纵比🔒)缩放图像(像素)")
            args_mode.value.push("nor-scale")
            args_g2_nest.value.push(["-scale", "", ""])
            break;
        case "-gravity":
            anchor_point.value = true
            args_item_title.value.push("设置基准锚点(左上角)")
            args_mode.value.push("lt-gravity")
            args_g2_nest.value.push(["-gravity", ""])
            break;
        case "-rotate":
            args_item_title.value.push("旋转图片(顺时针|-逆时针)")
            args_mode.value.push("-rotate")
            args_g2_nest.value.push(["-rotate", ""])
            break;
        case "-crop":
            if (componentList.value.length > 0 && getAt(componentList.value, -1).type === "-gravity") {
                args_item_title.value.push("裁剪图像")
            }
            else {
                args_item_title.value.push("裁剪图像(无锚点:默认左上角)")
            }
            args_mode.value.push("lt-crop")
            args_g2_nest.value.push(["-crop", "", "", "", ""])
            break;
        case "-flip":
            args_item_title.value.push("翻转图片(上下翻转)")
            args_mode.value.push("-flip")
            args_g2_nest.value.push(["-flip"])
            break;
        default:
            break;
    }

    // 创建组件对象并添加到列表
    const newComponent = {
        type: type,
        id: Date.now() // 使用时间戳作为唯一 ID
    };
    componentList.value.push(newComponent);
}
const addAudComponent = (type: string) => {
    // if (componentList.value.length > 0) {
    //     if (getAt(componentList.value, -1).type === "-gravity" && magick_args["before-need-gravity"].includes(type)) {
    //         anchor_point.value = false;
    //     } else if (getAt(componentList.value, -1).type === "-gravity") {
    //         anchor_point.value = false;
    //         args_g2_nest.value.pop()
    //         args_item_title.value.pop()
    //         args_mode.value.pop()
    //         componentList.value.pop()
    //     }
    // if (getAt(componentList.value, -1).type === "-background" && magick_args["before-need-background"].includes(type)) {
    //     anchor_point.value = false;
    // } else if (getAt(componentList.value, -1).type === "-background") {
    //     anchor_point.value = false;
    //     args_g2_nest.value.pop()
    //     args_item_title.value.pop()
    //     args_mode.value.pop()
    //     componentList.value.pop()
    // }
    // }
    switch (type) {
        case "__bash__":
            args_item_title.value.push("命令行")
            args_mode.value.push("__bash__")
            args_g2_nest.value.push(["__bash__", ""])
            break;
        case "-resize":
            args_item_title.value.push("(横纵比🔒)调整图像尺寸(像素)")
            args_mode.value.push("nor-resize")
            args_g2_nest.value.push(["-resize", "", ""])
            break;
        case "-scale":
            args_item_title.value.push("(横纵比🔒)缩放图像(像素)")
            args_mode.value.push("nor-scale")
            args_g2_nest.value.push(["-scale", "", ""])
            break;
        case "-gravity":
            anchor_point.value = true
            args_item_title.value.push("设置基准锚点(左上角)")
            args_mode.value.push("lt-gravity")
            args_g2_nest.value.push(["-gravity", ""])
            break;
        case "-rotate":
            args_item_title.value.push("旋转图片(顺时针|-逆时针)")
            args_mode.value.push("-rotate")
            args_g2_nest.value.push(["-rotate", ""])
            break;
        case "-crop":
            if (componentList.value.length > 0 && getAt(componentList.value, -1).type === "-gravity") {
                args_item_title.value.push("裁剪图像")
            }
            else {
                args_item_title.value.push("裁剪图像(无锚点:默认左上角)")
            }
            args_mode.value.push("lt-crop")
            args_g2_nest.value.push(["-crop", "", "", "", ""])
            break;
        case "-flip":
            args_item_title.value.push("翻转图片(上下翻转)")
            args_mode.value.push("-flip")
            args_g2_nest.value.push(["-flip"])
            break;
        default:
            break;
    }

    // 创建组件对象并添加到列表
    const newComponent = {
        type: type,
        id: Date.now() // 使用时间戳作为唯一 ID
    };
    componentList.value.push(newComponent);
}
const addOfficeComponent = (type: string) => {
    switch (type) {
        case "__bash__":
            args_item_title.value.push("命令行")
            args_mode.value.push("__bash__")
            args_g2_nest.value.push(["__bash__", ""])
            break;
        case "-resize":
            args_item_title.value.push("(横纵比🔒)调整图像尺寸(像素)")
            args_mode.value.push("nor-resize")
            args_g2_nest.value.push(["-resize", "", ""])
            break;
        case "-scale":
            args_item_title.value.push("(横纵比🔒)缩放图像(像素)")
            args_mode.value.push("nor-scale")
            args_g2_nest.value.push(["-scale", "", ""])
            break;
        case "-gravity":
            anchor_point.value = true
            args_item_title.value.push("设置基准锚点(左上角)")
            args_mode.value.push("lt-gravity")
            args_g2_nest.value.push(["-gravity", ""])
            break;
        case "-rotate":
            args_item_title.value.push("旋转图片(顺时针|-逆时针)")
            args_mode.value.push("-rotate")
            args_g2_nest.value.push(["-rotate", ""])
            break;
        case "-crop":
            if (componentList.value.length > 0 && getAt(componentList.value, -1).type === "-gravity") {
                args_item_title.value.push("裁剪图像")
            }
            else {
                args_item_title.value.push("裁剪图像(无锚点:默认左上角)")
            }
            args_mode.value.push("lt-crop")
            args_g2_nest.value.push(["-crop", "", "", "", ""])
            break;
        case "-flip":
            args_item_title.value.push("翻转图片(上下翻转)")
            args_mode.value.push("-flip")
            args_g2_nest.value.push(["-flip"])
            break;
        default:
            break;
    }

    // 创建组件对象并添加到列表
    const newComponent = {
        type: type,
        id: Date.now() // 使用时间戳作为唯一 ID
    };
    componentList.value.push(newComponent);
}

const closeItem = (index: number) => {
    if (getAt(componentList.value, -1).type === "-gravity") {
        anchor_point.value = false;
    }
    args_g2_nest.value.splice(index, 1)
    args_item_title.value.splice(index, 1)
    args_mode.value.splice(index, 1)
    componentList.value.splice(index, 1) // 直接从数组中移除该项
    if (getAt(componentList.value, -1).type === "-gravity") {
        anchor_point.value = true;
    }
}

// 清空所有操作
const clearComponents = () => {
    args_g2_nest.value = [];
    args_item_title.value = [];
    args_mode.value = [];
    componentList.value = [];
    anchor_point.value = false;
}

const updateGravity = (index: number, value: string) => {
    // 更新 args_g2_nest 对应位置的值
    args_g2_nest.value[index][1] = value;
    switch (value) {
        case "NorthWest":
            args_item_title.value[index] = "设置基准锚点(左上角)"
            break;
        case "North":
            args_item_title.value[index] = "设置基准锚点(顶部居中)"
            break;
        case "NorthEast":
            args_item_title.value[index] = "设置基准锚点(右上角)"
            break;
        case "West":
            args_item_title.value[index] = "设置基准锚点(左侧居中)"
            break;
        case "Center":
            args_item_title.value[index] = "设置基准锚点(正中心)"
            break;
        case "East":
            args_item_title.value[index] = "设置基准锚点(右侧居中)"
            break;
        case "SouthWest":
            args_item_title.value[index] = "设置基准锚点(左下角)"
            break;
        case "South":
            args_item_title.value[index] = "设置基准锚点(底部居中)"
            break;
        case "SouthEast":
            args_item_title.value[index] = "设置基准锚点(右下角)"
            break;
        default:
            break;

    }
    // nextTick(() => {
    //     // 强制刷新组件
    // });

    // 可以在这里做其他操作，比如记录日志或者触发其他逻辑
    console.log(`${args_g2_nest.value[index]} --- ${index}:`, value);
};

</script>
<style scoped>
.format-conversion-container {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: start;
    flex-direction: column;
}

.order-and-replace-name {
    font-size: 2vmin;
    display: grid;
    width: 100%;
    height: 4vmin;
    grid-template-columns: 5% 5% 5% 5% 5% 5% 55% 15%;
    border-bottom: 1px dashed var(--unite-but-color);
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

.text {
    display: flex;
    justify-content: center;
    align-items: center;
}

.conversion-item {
    display: grid;
    width: 100%;
    height: 4vmin;
    grid-template-columns: 45% 10% 45%;
    height: 4vmin;
    width: 100%;
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
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
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

.place-holder {
    width: 100%;
    height: 4vmin;
    font-size: medium;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--font-color);

}

.configer {
    flex: 1;
    display: flex;
    justify-content: start;
    align-items: start;
    flex-direction: row;
    width: 100%;
    border: 2px dashed var(--unite-but-color);
    border-radius: 1vmin;
    overflow-y: hidden;

    &::-webkit-scrollbar {
        width: 1vmin;
        /* 垂直滚动条宽度 */
        height: 1vmin;
        /* 水平滚动条高度 */

    }
}

.items-list {
    width: 20%;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: var(--title-bar-lg-1);
    border-right: 0.5vmin dashed var(--main-border);

    & ul {
        list-style: none;
        padding: 0;
        margin: 0;

        & .choose-item {
            padding: 0;
            height: 3vmin;
            margin: 1vmin 2vmin;
            border: 0.2vmin solid var(--normal-attention-color);
            background-color: var(--main-back-ground);
            border-radius: 1vmin;
            transition: box-shadow 0.5s ease-in-out;
            user-select: none;
            -webkit-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;

            &:hover {
                border: 0.15vmin solid var(--active-attention-color);
                box-shadow: 0 0 2vmin var(--normal-attention-color);
            }

            &:active {
                background-color: var(--normal-attention-color);
                border: 0.15vmin solid var(--active-attention-color);
                box-shadow: 0 0 2vmin var(--normal-attention-color);
                animation: active-icon 0.25s forwards;
                animation-timing-function: linear;
            }
        }

        & .anchor-point {
            border: 0.2vmin solid var(--title-max-icon-hover-shadow);
            background-color: var(--title-max-icon-shadow);

        }

        & .claen-oper {
            padding: 0;
            height: 3vmin;
            margin: 1vmin 2vmin;
            border: 0.2vmin solid var(--title-close-icon-hover-shadow);
            background-color: var(--title-close-icon-shadow);
            border-radius: 1vmin;
            transition: box-shadow 0.5s ease-in-out;
            user-select: none;
            -webkit-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;

            &:hover {
                border: 0.15vmin solid var(--title-close-icon-active-shadow);
                box-shadow: 0 0 2vmin var(--title-close-icon-shadow);
            }

            &:active {
                background-color: var(--title-close-icon-hover-shadow);
                border: 0.15vmin solid var(--title-close-icon-active-shadow);
                box-shadow: 0 0 2vmin var(--title-close-icon-shadow);
                animation: active-icon 0.25s forwards;
                animation-timing-function: linear;
            }
        }
    }

}



.items-group {
    flex: 1;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.item {
    flex: 1;
    width: 100%;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.title {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 4vmin;
    border-bottom: 1px dashed var(--unite-but-color);
}

.name {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 4vmin;
    border-top: 2px dashed var(--main-border);
    border-bottom: 1px dashed var(--unite-but-color);
}


.w-box {
    width: 100%;
}

.poer {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 4vmin;
    border-bottom: 1px dashed var(--unite-but-color);

    input {
        flex: 1;
        font-family: "楷体", 'Courier New', Courier, monospace;
        font-size: 2vmin;
        margin-left: auto;
        height: 4vmin;
        text-align: center;
        background-color: transparent;
        border: 0px dashed var(--unite-but-color);
        border-bottom: 2px dashed var(--unite-but-color);

        &:hover {
            border-color: var(--positive-show-color);
        }

        &:focus {
            outline: none;
            /* 取消默认的焦点轮廓 */
            /* border: 1px solid var(--active-attention-color); */
            border-bottom: 2px dashed var(--active-attention-color);
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);

        }
    }
}

.choose-anchor-point {
    width: 100%;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.gravity-item {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    height: 4vmin;
    border-bottom: 1px dashed var(--unite-but-color);
}

.div-g-ele {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    width: 3vmin;
    margin: 0 1vmin;
    height: 3vmin;
    border: 0.2vmin solid var(--normal-attention-color);
    border-radius: 50%;
    background-color: var(--main-back-ground);
    position: relative;
    cursor: pointer;
    transition: all 0.3s ease;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;

    &:hover {
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
    }

    &:active {
        background-color: var(--normal-attention-color);
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
        animation: active-icon 0.25s forwards;
        animation-timing-function: linear;
    }

    &.choose-radio {
        background-color: var(--normal-attention-color);
        border: 0.15vmin solid var(--active-attention-color);
    }

    &.choose-radio::before {
        content: '';
        position: absolute;
        top: 1.2vmin;
        left: 0.15vmin;
        width: 0.4vmin;
        height: 0.4vmin;
        border-radius: 1vmin;
        background-color: #fff;
    }

    &.choose-radio::after {
        content: '';
        position: absolute;
        top: 0.3vmin;
        left: 0.3vmin;
        width: 0.75vmin;
        height: 0.5vmin;
        border-radius: 1vmin;
        background-color: #fff;
        transform: rotate(-45deg);

    }
}


.single-input {
    width: 100%;
    font-family: "楷体", 'Courier New', Courier, monospace;
    font-size: 1.5vmin;
    margin-left: auto;
    height: 4vmin;
    text-align: center;
    background-color: transparent;
    border: 0px dashed var(--unite-but-color);
    border-bottom: 2px dashed var(--unite-but-color);

    &:hover {
        border-color: var(--positive-show-color);
    }

    &:focus {
        font-size: 2vmin;
        outline: none;
        /* 取消默认的焦点轮廓 */
        /* border: 1px solid var(--active-attention-color); */
        border-bottom: 2px dashed var(--active-attention-color);
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);

    }
}

.inline-button {
    width: 15%;
    margin-right: 2vmin;
    margin-left: auto;
    padding: 0 0.5vmin;
    border: 0.2vmin solid var(--normal-attention-color);
    background-color: var(--main-back-ground);
    border-radius: 1vmin;
    transition: box-shadow 0.5s ease-in-out;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;

    &:hover {
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
    }

    &:active {
        background-color: var(--normal-attention-color);
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
        animation: active-icon 0.25s forwards;
        animation-timing-function: linear;
    }
}



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
</style>