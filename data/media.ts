export type MediaType = 'book' | 'movie' | 'anime' | 'game' | 'song'
export type MediaState = 'done' | 'doing' | 'todo'

export interface MediaRecord {
  name: string
  creator?: string
  state?: MediaState
  date?: string
  note?: string
  lang?: string
}

export const book: MediaRecord[] = [
  {
    name: 'コンビニ人間',
    creator: '村田沙耶香',
  },
  {
    name: 'Do Androids Dream of Electric Sheep?',
    creator: 'Philip K. Dick',
  },
  {
    name: '化物語(上)',
    creator: '西尾維新',
  },
  {
    name: '化物語(下)',
    creator: '西尾維新',
  },
  {
    name: '君の話',
    creator: '三秋縋',
  },
  {
    name: '恋する寄生虫',
    creator: '三秋縋',
  },
  {
    name: 'あおぞらとくもりぞら',
    creator: '三秋縋',
  },
  {
    name: '三日間の幸福',
    creator: '三秋縋',
  },
  {
    name: 'スターティング・オーバー',
    creator: '三秋縋',
  },
  {
    name: '騎士の国の最弱英雄 バーガント反英雄譚1',
    creator: '八街歩',
  },
  {
    name: 'Braiding Sweetgrass',
    creator: 'Robin Wall Kimmerer',
  },
]

export const movie: MediaRecord[] = [
  {
    name: 'Interstellar',
    creator: 'Christopher Nolan',
  },
  {
    name: 'The Imitation Game',
    creator: 'Morten Tyldum',
  },
  {
    name: 'HER',
    creator: 'Spike Jonze',
  },
]

export const anime: MediaRecord[] = [
  {
    name: '「鬼滅の刃」無限城編',
    creator: 'Ufotable',
  },
  {
    name: '君の名は。',
    creator: '新海誠',
  },
  {
    name: 'ほしのこえ',
    creator: '新海誠',
  },
]

export const game: MediaRecord[] = [
  {
    name: 'There is no Game',
    creator: 'Kazuma Kondou',
  },
]

export const song = [
]

export const media: Record<MediaType, MediaRecord[]> = {
  book,
  movie,
  anime,
  game,
  song,
}
